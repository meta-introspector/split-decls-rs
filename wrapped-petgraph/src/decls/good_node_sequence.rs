macro_rules! deps {
    () => {
        NodeIndex!();
        FasNode!();
        FasNodeContainer!();
        FasNodeIndex!();
        NodeLinkedList!();
        Buckets!();
    };
}

macro_rules! good_node_sequence {
    () => {
        deps!();
        fn good_node_sequence (edge_refs : impl Iterator < Item = (NodeIndex < usize > , NodeIndex < usize >) > ,) -> HashMap < usize , usize > { let mut nodes = FasNodeContainer { nodes : Vec :: new () } ; let mut buckets = Buckets { sinks_or_isolated : NodeLinkedList :: new () , sources : NodeLinkedList :: new () , bidirectional_pve_dd : Vec :: new () , bidirectional_nve_dd : Vec :: new () , } ; let mut graph_ix_lookup = HashMap :: new () ; for (from_g_ix , to_g_ix) in edge_refs { let mut fas_node_entry = | g_ix : NodeIndex < usize > | -> FasNodeIndex { match graph_ix_lookup . get (& g_ix) { Some (fas_ix) => * fas_ix , None => { let fas_ix = FasNodeIndex (nodes . nodes . len ()) ; nodes . nodes . push (LinkedListEntry :: new (FasNode { graph_ix : g_ix , out_edges : Vec :: new () , in_edges : Vec :: new () , out_degree : 0 , in_degree : 0 , })) ; graph_ix_lookup . insert (g_ix , fas_ix) ; fas_ix } } } ; let from_fas_ix = fas_node_entry (from_g_ix) ; let to_fas_ix = fas_node_entry (to_g_ix) ; nodes [from_fas_ix] . data () . out_edges . push (to_fas_ix) ; nodes [to_fas_ix] . data () . in_edges . push (from_fas_ix) ; } for entry in nodes . nodes . iter_mut () { let node = entry . data () ; node . out_degree = node . out_edges . len () ; node . in_degree = node . in_edges . len () ; } for i in 0 .. nodes . nodes . len () { let fas_ix = FasNodeIndex (i) ; buckets . suitable_bucket (fas_ix , & mut nodes) . push_front (fas_ix , & mut nodes) ; } let mut s_1 = VecDeque :: new () ; let mut s_2 = VecDeque :: new () ; loop { let mut some_moved = false ; while let Some (sink_fas_ix) = buckets . sinks_or_isolated . pop (& mut nodes) { some_moved = true ; buckets . update_neighbour_node_buckets (sink_fas_ix , & mut nodes) ; s_2 . push_front (nodes [sink_fas_ix] . data () . graph_ix) ; } while let Some (source_fas_ix) = buckets . sources . pop (& mut nodes) { some_moved = true ; buckets . update_neighbour_node_buckets (source_fas_ix , & mut nodes) ; s_1 . push_back (nodes [source_fas_ix] . data () . graph_ix) ; } if let Some (list) = buckets . bidirectional_pve_dd . iter_mut () . rev () . chain (buckets . bidirectional_nve_dd . iter_mut ()) . find (| b | b . start . is_some ()) { let highest_dd_fas_ix = list . pop (& mut nodes) . unwrap () ; some_moved = true ; buckets . update_neighbour_node_buckets (highest_dd_fas_ix , & mut nodes) ; s_1 . push_back (nodes [highest_dd_fas_ix] . data () . graph_ix) ; Buckets :: trim_bucket_list (& mut buckets . bidirectional_pve_dd) ; Buckets :: trim_bucket_list (& mut buckets . bidirectional_nve_dd) ; } if ! some_moved { break ; } } s_1 . into_iter () . chain (s_2) . enumerate () . map (| (seq_order , node_index) | (node_index . index () , seq_order)) . collect () }
    };
}

good_node_sequence!()