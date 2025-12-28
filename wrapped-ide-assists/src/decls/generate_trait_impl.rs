macro_rules! generate_trait_impl {
    () => {
        # [doc = " Generates the corresponding `impl <trait> for Type {}` including type"] # [doc = " and lifetime parameters, with `<trait>` appended to `impl`'s generic parameters' bounds."] # [doc = ""] # [doc = " This is useful for traits like `PartialEq`, since `impl<T> PartialEq for U<T>` often requires `T: PartialEq`."] pub (crate) fn generate_trait_impl (is_unsafe : bool , adt : & ast :: Adt , trait_ : ast :: Type) -> ast :: Impl { generate_impl_inner (is_unsafe , adt , Some (trait_) , true , None) }
    };
}

generate_trait_impl!()