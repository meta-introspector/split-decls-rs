macro_rules! generate_impl_with_item {
    () => {
        # [doc = " Generates the corresponding `impl Type {}` including type and lifetime"] # [doc = " parameters."] pub (crate) fn generate_impl_with_item (adt : & ast :: Adt , body : Option < ast :: AssocItemList > ,) -> ast :: Impl { generate_impl_inner (false , adt , None , true , body) }
    };
}

generate_impl_with_item!()