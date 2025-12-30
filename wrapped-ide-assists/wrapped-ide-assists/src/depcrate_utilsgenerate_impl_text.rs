// Generated macro for generate_impl_text (function)
macro_rules! Depcrate_utilsgenerate_impl_text {
() => {
// Module: crate::utils
// Provides: {"generate_impl_text"}
// Dependencies: {}
# [doc = " Generates the surrounding `impl Type { <code> }` including type and lifetime"] # [doc = " parameters."] pub (crate) fn generate_impl_text (adt : & ast :: Adt , code : & str) -> String { generate_impl_text_inner (adt , None , true , code) }
};
}
