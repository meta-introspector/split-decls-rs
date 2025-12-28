macro_rules! deps {
    () => {
        Style!();
        Edge!();
    };
}

macro_rules! edge {
    () => {
        deps!();
        fn edge (from : usize , to : usize , label : & 'static str , style : Style) -> Edge { Edge { from , to , label , style } }
    };
}

edge!();