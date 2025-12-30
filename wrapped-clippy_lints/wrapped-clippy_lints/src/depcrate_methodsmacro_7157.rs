// Generated macro for macro_7157 (macro)
macro_rules! Depcrate_methodsmacro_7157 {
() => {
// Module: crate::methods
// Provides: {"macro_7157"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of the `offset` pointer method with a `usize` casted to an"] # [doc = " `isize`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " If we’re always increasing the pointer address, we can avoid the numeric"] # [doc = " cast by using the `add` method instead."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let vec = vec![b'a', b'b', b'c'];"] # [doc = " let ptr = vec.as_ptr();"] # [doc = " let offset = 1_usize;"] # [doc = ""] # [doc = " unsafe {"] # [doc = "     ptr.offset(offset as isize);"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Could be written:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " let vec = vec![b'a', b'b', b'c'];"] # [doc = " let ptr = vec.as_ptr();"] # [doc = " let offset = 1_usize;"] # [doc = ""] # [doc = " unsafe {"] # [doc = "     ptr.add(offset);"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.30.0"] pub PTR_OFFSET_WITH_CAST , complexity , "unneeded pointer offset cast" }
};
}
