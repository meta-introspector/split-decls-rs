// Generated macro for macro_4984 (macro)
macro_rules! Depcrate_matchesmacro_4984 {
() => {
// Module: crate::matches
// Provides: {"macro_4984"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Finds nested `match` or `if let` expressions where the patterns may be \"collapsed\" together"] # [doc = " without adding any branches."] # [doc = ""] # [doc = " Note that this lint is not intended to find _all_ cases where nested match patterns can be merged, but only"] # [doc = " cases where merging would most likely make the code more readable."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It is unnecessarily verbose and complex."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn func(opt: Option<Result<u64, String>>) {"] # [doc = "     let n = match opt {"] # [doc = "         Some(n) => match n {"] # [doc = "             Ok(n) => n,"] # [doc = "             _ => return,"] # [doc = "         }"] # [doc = "         None => return,"] # [doc = "     };"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " fn func(opt: Option<Result<u64, String>>) {"] # [doc = "     let n = match opt {"] # [doc = "         Some(Ok(n)) => n,"] # [doc = "         _ => return,"] # [doc = "     };"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.50.0"] pub COLLAPSIBLE_MATCH , style , "Nested `match` or `if let` expressions where the patterns may be \"collapsed\" together." }
};
}
