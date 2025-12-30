// Generated macro for macro_7873 (macro)
macro_rules! Depcrate_needless_for_eachmacro_7873 {
() => {
// Module: crate::needless_for_each
// Provides: {"macro_7873"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `for_each` that would be more simply written as a"] # [doc = " `for` loop."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `for_each` may be used after applying iterator transformers like"] # [doc = " `filter` for better readability and performance. It may also be used to fit a simple"] # [doc = " operation on one line."] # [doc = " But when none of these apply, a simple `for` loop is more idiomatic."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let v = vec![0, 1, 2];"] # [doc = " v.iter().for_each(|elem| {"] # [doc = "     println!(\"{elem}\");"] # [doc = " })"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let v = vec![0, 1, 2];"] # [doc = " for elem in &v {"] # [doc = "     println!(\"{elem}\");"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " ### Known Problems"] # [doc = " When doing things such as:"] # [doc = " ```ignore"] # [doc = " let v = vec![0, 1, 2];"] # [doc = " v.iter().for_each(|elem| unsafe {"] # [doc = "     libc::printf(c\"%d\\n\".as_ptr(), elem);"] # [doc = " });"] # [doc = " ```"] # [doc = " This lint will not trigger."] # [clippy :: version = "1.53.0"] pub NEEDLESS_FOR_EACH , pedantic , "using `for_each` where a `for` loop would be simpler" }
};
}
