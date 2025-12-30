// Generated macro for macro_10951 (macro)
macro_rules! Depcrate_unnecessary_struct_initializationmacro_10951 {
() => {
// Module: crate::unnecessary_struct_initialization
// Provides: {"macro_10951"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for initialization of an identical `struct` from another instance"] # [doc = " of the type, either by copying a base without setting any field or by"] # [doc = " moving all fields individually."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Readability suffers from unnecessary struct building."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " struct S { s: String }"] # [doc = ""] # [doc = " let a = S { s: String::from(\"Hello, world!\") };"] # [doc = " let b = S { ..a };"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " struct S { s: String }"] # [doc = ""] # [doc = " let a = S { s: String::from(\"Hello, world!\") };"] # [doc = " let b = a;"] # [doc = " ```"] # [doc = ""] # [doc = " The struct literal ``S { ..a }`` in the assignment to ``b`` could be replaced"] # [doc = " with just ``a``."] # [doc = ""] # [doc = " ### Known Problems"] # [doc = " Has false positives when the base is a place expression that cannot be"] # [doc = " moved out of, see [#10547](https://github.com/rust-lang/rust-clippy/issues/10547)."] # [doc = ""] # [doc = " Empty structs are ignored by the lint."] # [clippy :: version = "1.70.0"] pub UNNECESSARY_STRUCT_INITIALIZATION , nursery , "struct built from a base that can be written mode concisely" }
};
}
