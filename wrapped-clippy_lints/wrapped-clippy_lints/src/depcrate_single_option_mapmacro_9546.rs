// Generated macro for macro_9546 (macro)
macro_rules! Depcrate_single_option_mapmacro_9546 {
() => {
// Module: crate::single_option_map
// Provides: {"macro_9546"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for functions with method calls to `.map(_)` on an arg"] # [doc = " of type `Option` as the outermost expression."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Taking and returning an `Option<T>` may require additional"] # [doc = " `Some(_)` and `unwrap` if all you have is a `T`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn double(param: Option<u32>) -> Option<u32> {"] # [doc = "     param.map(|x| x * 2)"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " fn double(param: u32) -> u32 {"] # [doc = "     param * 2"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.87.0"] pub SINGLE_OPTION_MAP , nursery , "Checks for functions with method calls to `.map(_)` on an arg of type `Option` as the outermost expression." }
};
}
