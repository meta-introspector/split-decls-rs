// Generated macro for macro_9882 (macro)
macro_rules! Depcrate_size_of_refmacro_9882 {
() => {
// Module: crate::size_of_ref
// Provides: {"macro_9882"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = ""] # [doc = " Checks for calls to `size_of_val()` where the argument is"] # [doc = " a reference to a reference."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = ""] # [doc = " Calling `size_of_val()` with a reference to a reference as the argument"] # [doc = " yields the size of the reference-type, not the size of the value behind"] # [doc = " the reference."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " struct Foo {"] # [doc = "     buffer: [u8],"] # [doc = " }"] # [doc = ""] # [doc = " impl Foo {"] # [doc = "     fn size(&self) -> usize {"] # [doc = "         // Note that `&self` as an argument is a `&&Foo`: Because `self`"] # [doc = "         // is already a reference, `&self` is a double-reference."] # [doc = "         // The return value of `size_of_val()` therefore is the"] # [doc = "         // size of the reference-type, not the size of `self`."] # [doc = "         size_of_val(&self)"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " struct Foo {"] # [doc = "     buffer: [u8],"] # [doc = " }"] # [doc = ""] # [doc = " impl Foo {"] # [doc = "     fn size(&self) -> usize {"] # [doc = "         // Correct"] # [doc = "         size_of_val(self)"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.68.0"] pub SIZE_OF_REF , suspicious , "Argument to `size_of_val()` is a double-reference, which is almost certainly unintended" }
};
}
