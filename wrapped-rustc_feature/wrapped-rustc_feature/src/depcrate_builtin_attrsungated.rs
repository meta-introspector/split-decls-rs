// Generated macro for ungated (macro)
macro_rules! Depcrate_builtin_attrsungated {
() => {
// Module: crate::builtin_attrs
// Provides: {"ungated"}
// Dependencies: {}
macro_rules ! ungated { (unsafe ($ edition : ident) $ attr : ident , $ typ : expr , $ tpl : expr , $ duplicates : expr , $ encode_cross_crate : expr $ (,) ?) => { BuiltinAttribute { name : sym ::$ attr , encode_cross_crate : $ encode_cross_crate , type_ : $ typ , safety : AttributeSafety :: Unsafe { unsafe_since : Some (Edition ::$ edition) } , template : $ tpl , gate : Ungated , duplicates : $ duplicates , } } ; (unsafe $ attr : ident , $ typ : expr , $ tpl : expr , $ duplicates : expr , $ encode_cross_crate : expr $ (,) ?) => { BuiltinAttribute { name : sym ::$ attr , encode_cross_crate : $ encode_cross_crate , type_ : $ typ , safety : AttributeSafety :: Unsafe { unsafe_since : None } , template : $ tpl , gate : Ungated , duplicates : $ duplicates , } } ; ($ attr : ident , $ typ : expr , $ tpl : expr , $ duplicates : expr , $ encode_cross_crate : expr $ (,) ?) => { BuiltinAttribute { name : sym ::$ attr , encode_cross_crate : $ encode_cross_crate , type_ : $ typ , safety : AttributeSafety :: Normal , template : $ tpl , gate : Ungated , duplicates : $ duplicates , } } ; }
};
}
