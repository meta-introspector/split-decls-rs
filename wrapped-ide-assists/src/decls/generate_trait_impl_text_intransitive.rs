macro_rules! generate_trait_impl_text_intransitive {
    () => {
        # [doc = " Generates the surrounding `impl <trait> for Type { <code> }` including type"] # [doc = " and lifetime parameters, with `impl`'s generic parameters' bounds kept as-is."] # [doc = ""] # [doc = " This is useful for traits like `From<T>`, since `impl<T> From<T> for U<T>` doesn't require `T: From<T>`."] pub (crate) fn generate_trait_impl_text_intransitive (adt : & ast :: Adt , trait_text : & str , code : & str ,) -> String { generate_impl_text_inner (adt , Some (trait_text) , false , code) }
    };
}

generate_trait_impl_text_intransitive!()