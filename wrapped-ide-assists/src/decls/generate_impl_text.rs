macro_rules! generate_impl_text {
    () => {
        # [doc = " Generates the surrounding `impl Type { <code> }` including type and lifetime"] # [doc = " parameters."] pub (crate) fn generate_impl_text (adt : & ast :: Adt , code : & str) -> String { generate_impl_text_inner (adt , None , true , code) }
    };
}

generate_impl_text!()