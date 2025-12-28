macro_rules! deps {
    () => {
        Digest!();
    };
}

macro_rules! implement {
    () => {
        deps!();
        macro_rules ! implement { ($ kind : ident , $ format : expr) => { impl core :: fmt ::$ kind for Digest { fn fmt (& self , formatter : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { for value in & self . 0 { write ! (formatter , $ format , value) ?; } Ok (()) } } } ; }
    };
}

implement!()