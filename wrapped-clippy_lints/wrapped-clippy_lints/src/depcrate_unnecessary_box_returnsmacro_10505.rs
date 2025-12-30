// Generated macro for macro_10505 (macro)
macro_rules! Depcrate_unnecessary_box_returnsmacro_10505 {
() => {
// Module: crate::unnecessary_box_returns
// Provides: {"macro_10505"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = ""] # [doc = " Checks for a return type containing a `Box<T>` where `T` implements `Sized`"] # [doc = ""] # [doc = " The lint ignores `Box<T>` where `T` is larger than `unnecessary_box_size`,"] # [doc = " as returning a large `T` directly may be detrimental to performance."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = ""] # [doc = " It's better to just return `T` in these cases. The caller may not need"] # [doc = " the value to be boxed, and it's expensive to free the memory once the"] # [doc = " `Box<T>` been dropped."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn foo() -> Box<String> {"] # [doc = "     Box::new(String::from(\"Hello, world!\"))"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " fn foo() -> String {"] # [doc = "     String::from(\"Hello, world!\")"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.70.0"] pub UNNECESSARY_BOX_RETURNS , pedantic , "Needlessly returning a Box" }
};
}
