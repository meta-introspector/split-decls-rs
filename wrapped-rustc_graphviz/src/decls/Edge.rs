macro_rules! deps {
    () => {
        Style!();
    };
}

macro_rules! Edge {
    () => {
        deps!();
        struct Edge { from : usize , to : usize , label : & 'static str , style : Style , }
    };
}

Edge!();