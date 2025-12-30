// Generated macro for macro_9130 (macro)
macro_rules! Depcrate_redundant_pub_cratemacro_9130 {
() => {
// Module: crate::redundant_pub_crate
// Provides: {"macro_9130"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for items declared `pub(crate)` that are not crate visible because they"] # [doc = " are inside a private module."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Writing `pub(crate)` is misleading when it's redundant due to the parent"] # [doc = " module's visibility."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " mod internal {"] # [doc = "     pub(crate) fn internal_fn() { }"] # [doc = " }"] # [doc = " ```"] # [doc = " This function is not visible outside the module and it can be declared with `pub` or"] # [doc = " private visibility"] # [doc = " ```no_run"] # [doc = " mod internal {"] # [doc = "     pub fn internal_fn() { }"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.44.0"] pub REDUNDANT_PUB_CRATE , nursery , "Using `pub(crate)` visibility on items that are not crate visible due to the visibility of the module that contains them." }
};
}
