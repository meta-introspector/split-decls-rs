macro_rules! impl_load {
    () => {
        macro_rules ! impl_load { ($ type_alias : ty , $ type_alias_expr : ident , $ conv_function : ident , $ func_name : ident) => { # [doc = " Convert bytes in `src` to a given primitive."] pub fn $ func_name (src : & [u8]) -> $ type_alias { assert_eq ! (mem :: size_of ::<$ type_alias > () , src . len ()) ; $ type_alias_expr ::$ conv_function (src . try_into () . unwrap ()) } } ; }
    };
}

impl_load!()