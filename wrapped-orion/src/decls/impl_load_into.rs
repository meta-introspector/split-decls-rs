macro_rules! impl_load_into {
    () => {
        macro_rules ! impl_load_into { ($ type_alias : ty , $ type_alias_expr : ident , $ conv_function : ident , $ func_name : ident) => { # [doc = " Load bytes in `src` into `dst`."] pub fn $ func_name (src : & [u8] , dst : & mut [$ type_alias]) { let type_alias_len = mem :: size_of ::<$ type_alias > () ; assert_eq ! (mem :: size_of_val (dst) , src . len ()) ; for (src_chunk , dst_elem) in src . chunks_exact (type_alias_len) . zip (dst . iter_mut ()) { debug_assert_eq ! (src_chunk . len () , type_alias_len) ; * dst_elem = $ type_alias_expr ::$ conv_function (src_chunk . try_into () . unwrap ()) ; } } } ; }
    };
}

impl_load_into!()