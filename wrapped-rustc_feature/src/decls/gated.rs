macro_rules! deps {
    () => {
        BuiltinAttribute!();
        Features!();
        AttributeSafety!();
    };
}

macro_rules! gated {
    () => {
        deps!();
        macro_rules ! gated { (unsafe $ attr : ident , $ typ : expr , $ tpl : expr , $ duplicates : expr , $ encode_cross_crate : expr , $ gate : ident , $ message : expr $ (,) ?) => { BuiltinAttribute { name : sym ::$ attr , encode_cross_crate : $ encode_cross_crate , type_ : $ typ , safety : AttributeSafety :: Unsafe { unsafe_since : None } , template : $ tpl , duplicates : $ duplicates , gate : Gated { feature : sym ::$ gate , message : $ message , check : Features ::$ gate , notes : & [] , } , } } ; (unsafe $ attr : ident , $ typ : expr , $ tpl : expr , $ duplicates : expr , $ encode_cross_crate : expr , $ message : expr $ (,) ?) => { BuiltinAttribute { name : sym ::$ attr , encode_cross_crate : $ encode_cross_crate , type_ : $ typ , safety : AttributeSafety :: Unsafe { unsafe_since : None } , template : $ tpl , duplicates : $ duplicates , gate : Gated { feature : sym ::$ attr , message : $ message , check : Features ::$ attr , notes : & [] , } , } } ; ($ attr : ident , $ typ : expr , $ tpl : expr , $ duplicates : expr , $ encode_cross_crate : expr , $ gate : ident , $ message : expr $ (,) ?) => { BuiltinAttribute { name : sym ::$ attr , encode_cross_crate : $ encode_cross_crate , type_ : $ typ , safety : AttributeSafety :: Normal , template : $ tpl , duplicates : $ duplicates , gate : Gated { feature : sym ::$ gate , message : $ message , check : Features ::$ gate , notes : & [] , } , } } ; ($ attr : ident , $ typ : expr , $ tpl : expr , $ duplicates : expr , $ encode_cross_crate : expr , $ message : expr $ (,) ?) => { BuiltinAttribute { name : sym ::$ attr , encode_cross_crate : $ encode_cross_crate , type_ : $ typ , safety : AttributeSafety :: Normal , template : $ tpl , duplicates : $ duplicates , gate : Gated { feature : sym ::$ attr , message : $ message , check : Features ::$ attr , notes : & [] , } , } } ; }
    };
}

gated!();