macro_rules! deps {
    () => {
        LabelledGraph!();
        Trivial!();
        Edge!();
        LabelledGraphWithEscStrs!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl LabelledGraphWithEscStrs { fn new (name : & 'static str , node_labels : Trivial , edges : Vec < Edge >) -> LabelledGraphWithEscStrs { LabelledGraphWithEscStrs { graph : LabelledGraph :: new (name , node_labels , edges , None) } } }
    };
}

impl_23!();