// Generated macro for macro_8796 (macro)
macro_rules! Depcrate_ptrmacro_8796 {
() => {
// Module: crate::ptr
// Provides: {"macro_8796"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " This lint checks for functions that take immutable references and return"] # [doc = " mutable ones. This will not trigger if no unsafe code exists as there"] # [doc = " are multiple safe functions which will do this transformation"] # [doc = ""] # [doc = " To be on the conservative side, if there's at least one mutable"] # [doc = " reference with the output lifetime, this lint will not trigger."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Creating a mutable reference which can be repeatably derived from an"] # [doc = " immutable reference is unsound as it allows creating multiple live"] # [doc = " mutable references to the same object."] # [doc = ""] # [doc = " This [error](https://github.com/rust-lang/rust/issues/39465) actually"] # [doc = " lead to an interim Rust release 1.15.1."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " This pattern is used by memory allocators to allow allocating multiple"] # [doc = " objects while returning mutable references to each one. So long as"] # [doc = " different mutable references are returned each time such a function may"] # [doc = " be safe."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```ignore"] # [doc = " fn foo(&Foo) -> &mut Bar { .. }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub MUT_FROM_REF , correctness , "fns that create mutable refs from immutable ref args" }
};
}
