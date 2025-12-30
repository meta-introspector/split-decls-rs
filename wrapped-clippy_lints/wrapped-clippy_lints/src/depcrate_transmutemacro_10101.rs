// Generated macro for macro_10101 (macro)
macro_rules! Depcrate_transmutemacro_10101 {
() => {
// Module: crate::transmute
// Provides: {"macro_10101"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = "Checks for transmutes that could be a pointer cast."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Readability. The code tricks people into thinking that"] # [doc = " something complex is going on."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # let p: *const [i32] = &[];"] # [doc = " unsafe { std::mem::transmute::<*const [i32], *const [u16]>(p) };"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let p: *const [i32] = &[];"] # [doc = " p as *const [u16];"] # [doc = " ```"] # [clippy :: version = "1.47.0"] pub TRANSMUTES_EXPRESSIBLE_AS_PTR_CASTS , complexity , "transmutes that could be a pointer cast" }
};
}
