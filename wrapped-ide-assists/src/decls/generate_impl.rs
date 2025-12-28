macro_rules! generate_impl {
    () => {
        pub (crate) fn generate_impl (adt : & ast :: Adt) -> ast :: Impl { generate_impl_inner (false , adt , None , true , None) }
    };
}

generate_impl!()