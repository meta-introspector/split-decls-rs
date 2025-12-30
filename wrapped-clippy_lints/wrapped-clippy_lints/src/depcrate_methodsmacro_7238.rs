// Generated macro for macro_7238 (macro)
macro_rules! Depcrate_methodsmacro_7238 {
() => {
// Module: crate::methods
// Provides: {"macro_7238"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for uses of `map` which return the original item."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `inspect` is both clearer in intent and shorter."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let x = Some(0).map(|x| { println!(\"{x}\"); x });"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let x = Some(0).inspect(|x| println!(\"{x}\"));"] # [doc = " ```"] # [clippy :: version = "1.81.0"] pub MANUAL_INSPECT , complexity , "use of `map` returning the original item" }
};
}
