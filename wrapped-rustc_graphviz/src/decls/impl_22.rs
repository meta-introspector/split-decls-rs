macro_rules! deps {
    () => {
        Style!();
        LabelledGraph!();
        Edge!();
        Trivial!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl LabelledGraph { fn new (name : & 'static str , node_labels : Trivial , edges : Vec < Edge > , node_styles : Option < Vec < Style > > ,) -> LabelledGraph { let count = node_labels . len () ; LabelledGraph { name , node_labels : node_labels . to_opt_strs () , edges , node_styles : match node_styles { Some (nodes) => nodes , None => vec ! [Style :: None ; count] , } , } } }
    };
}

impl_22!();