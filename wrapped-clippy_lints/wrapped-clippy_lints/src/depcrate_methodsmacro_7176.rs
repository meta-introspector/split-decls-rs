// Generated macro for macro_7176 (macro)
macro_rules! Depcrate_methodsmacro_7176 {
() => {
// Module: crate::methods
// Provides: {"macro_7176"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for unnecessary calls to [`ToOwned::to_owned`](https://doc.rust-lang.org/std/borrow/trait.ToOwned.html#tymethod.to_owned)"] # [doc = " and other `to_owned`-like functions."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The unnecessary calls result in useless allocations."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " `unnecessary_to_owned` can falsely trigger if `IntoIterator::into_iter` is applied to an"] # [doc = " owned copy of a resource and the resource is later used mutably. See"] # [doc = " [#8148](https://github.com/rust-lang/rust-clippy/issues/8148)."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let path = std::path::Path::new(\"x\");"] # [doc = " foo(&path.to_string_lossy().to_string());"] # [doc = " fn foo(s: &str) {}"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let path = std::path::Path::new(\"x\");"] # [doc = " foo(&path.to_string_lossy());"] # [doc = " fn foo(s: &str) {}"] # [doc = " ```"] # [clippy :: version = "1.59.0"] pub UNNECESSARY_TO_OWNED , perf , "unnecessary calls to `to_owned`-like functions" }
};
}
