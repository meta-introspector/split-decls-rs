macro_rules! map_node_range_up_rooted {
    () => {
        # [doc = " Maps up the text range out of the expansion hierarchy back into the original file its from only"] # [doc = " considering the root spans contained."] # [doc = " Unlike [`map_node_range_up`], this will not return `None` if any anchors or syntax contexts differ."] pub fn map_node_range_up_rooted (db : & dyn ExpandDatabase , exp_map : & ExpansionSpanMap , range : TextRange ,) -> Option < FileRange > { let mut spans = exp_map . spans_for_range (range) . filter (| span | span . ctx . is_root ()) ; let Span { range , anchor , ctx : _ } = spans . next () ? ; let mut start = range . start () ; let mut end = range . end () ; for span in spans { if span . anchor != anchor { return None ; } start = start . min (span . range . start ()) ; end = end . max (span . range . end ()) ; } let file_id = EditionedFileId :: from_span (db , anchor . file_id) ; let anchor_offset = db . ast_id_map (file_id . into ()) . get_erased (anchor . ast_id) . text_range () . start () ; Some (FileRange { file_id , range : TextRange :: new (start , end) + anchor_offset }) }
    };
}

map_node_range_up_rooted!()