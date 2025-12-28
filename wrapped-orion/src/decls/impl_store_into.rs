macro_rules! impl_store_into {
    () => {
        macro_rules ! impl_store_into { ($ type_alias : ty , $ conv_function : ident , $ func_name : ident) => { # [doc = " Store bytes in `src` in `dst`."] pub fn $ func_name (src : & [$ type_alias] , dst : & mut [u8]) { let type_alias_len = mem :: size_of ::<$ type_alias > () ; assert_eq ! (mem :: size_of_val (src) , dst . len ()) ; for (src_elem , dst_chunk) in src . iter () . zip (dst . chunks_exact_mut (type_alias_len)) { dst_chunk . copy_from_slice (& src_elem .$ conv_function ()) ; } } } ; }
    };
}

impl_store_into!();