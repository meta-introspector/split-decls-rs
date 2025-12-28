macro_rules! Tree {
    () => {
        struct Tree < T : Send > { value : T , children : Vec < Tree < T > > , }
    };
}

Tree!();