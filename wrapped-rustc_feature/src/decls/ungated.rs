macro_rules! deps {
    () => {
        BuiltinAttribute!();
        AttributeSafety!();
    };
}

macro_rules! ungated {
    () => {
        deps!();
        macro_rules ! ungated { (unsafe ($ edition : ident) $ attr : ident , $ typ : expr , $ tpl : expr , $ duplicates : expr , $ encode_cross_crate : expr $ (,) ?) => { BuiltinAttribute { name : sym ::$ attr , encode_cross_crate : $ encode_cross_crate , type_ : $ typ , safety : AttributeSafety :: Unsafe { unsafe_since : Some (Edition ::$ edition) } , template : $ tpl , gate : Ungated , duplicates : $ duplicates , } } ; (unsafe $ attr : ident , $ typ : expr , $ tpl : expr , $ duplicates : expr , $ encode_cross_crate : expr $ (,) ?) => { BuiltinAttribute { name : sym ::$ attr , encode_cross_crate : $ encode_cross_crate , type_ : $ typ , safety : AttributeSafety :: Unsafe { unsafe_since : None } , template : $ tpl , gate : Ungated , duplicates : $ duplicates , } } ; ($ attr : ident , $ typ : expr , $ tpl : expr , $ duplicates : expr , $ encode_cross_crate : expr $ (,) ?) => { BuiltinAttribute { name : sym ::$ attr , encode_cross_crate : $ encode_cross_crate , type_ : $ typ , safety : AttributeSafety :: Normal , template : $ tpl , gate : Ungated , duplicates : $ duplicates , } } ; }
    };
}

ungated!();