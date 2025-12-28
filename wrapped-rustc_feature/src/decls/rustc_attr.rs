macro_rules! deps {
    () => {
        BuiltinAttribute!();
        AttributeSafety!();
        Features!();
    };
}

macro_rules! rustc_attr {
    () => {
        deps!();
        macro_rules ! rustc_attr { (TEST , $ attr : ident , $ typ : expr , $ tpl : expr , $ duplicate : expr , $ encode_cross_crate : expr $ (,) ?) => { rustc_attr ! ($ attr , $ typ , $ tpl , $ duplicate , $ encode_cross_crate , concat ! ("the `#[" , stringify ! ($ attr) , "]` attribute is used for rustc unit tests") ,) } ; ($ attr : ident , $ typ : expr , $ tpl : expr , $ duplicates : expr , $ encode_cross_crate : expr , $ ($ notes : expr) ,* $ (,) ?) => { BuiltinAttribute { name : sym ::$ attr , encode_cross_crate : $ encode_cross_crate , type_ : $ typ , safety : AttributeSafety :: Normal , template : $ tpl , duplicates : $ duplicates , gate : Gated { feature : sym :: rustc_attrs , message : "use of an internal attribute" , check : Features :: rustc_attrs , notes : & [concat ! ("the `#[" , stringify ! ($ attr) , "]` attribute is an internal implementation detail that will never be stable") , $ ($ notes) ,*] } , } } ; }
    };
}

rustc_attr!()