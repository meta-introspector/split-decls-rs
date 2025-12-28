macro_rules! deps {
    () => {
        Edge!();
        EdgeRef!();
        Label!();
    };
}

macro_rules! find_join {
    () => {
        deps!();
        fn find_join < G , F > (graph : & G , edge : G :: EdgeRef , mate : & [Option < G :: NodeId >] , label : & mut [Label < G >] , first_inner : & mut [usize] , mut visitor : F ,) where G : IntoEdges + NodeIndexable + Visitable , F : FnMut (G :: NodeId) , { let source = graph . to_index (edge . source ()) ; let target = graph . to_index (edge . target ()) ; let mut left = first_inner [source] ; let mut right = first_inner [target] ; if left == right { return ; } let flag = Label :: Flag (edge . id ()) ; label [left] = flag ; label [right] = flag ; let join = loop { if right != graph . dummy_idx () { core :: mem :: swap (& mut left , & mut right) ; } let left_mate = graph . to_index (mate [left] . unwrap ()) ; let next_inner = label [left_mate] . to_vertex () . unwrap () ; left = first_inner [graph . to_index (next_inner)] ; if ! label [left] . is_flagged (edge . id ()) { label [left] = flag ; } else { break left ; } } ; for endpoint in [source , target] . iter () . copied () { let mut inner = first_inner [endpoint] ; while inner != join { if let Some (ix) = graph . try_from_index (inner) { visitor (ix) ; } label [inner] = Label :: Edge (edge . id () , [edge . source () , edge . target ()]) ; first_inner [inner] = join ; let inner_mate = graph . to_index (mate [inner] . unwrap ()) ; let next_inner = label [inner_mate] . to_vertex () . unwrap () ; inner = first_inner [graph . to_index (next_inner)] ; } } for (vertex_idx , vertex_label) in label . iter () . enumerate () { if vertex_idx != graph . dummy_idx () && vertex_label . is_outer () && label [first_inner [vertex_idx]] . is_outer () { first_inner [vertex_idx] = join ; } } }
    };
}

find_join!()