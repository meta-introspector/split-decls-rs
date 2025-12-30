// Generated macro for macro_3078 (macro)
macro_rules! Depcrate_init_numbered_fieldsmacro_3078 {
() => {
// Module: crate::init_numbered_fields
// Provides: {"macro_3078"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for tuple structs initialized with field syntax."] # [doc = " It will however not lint if a base initializer is present."] # [doc = " The lint will also ignore code in macros."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This may be confusing to the uninitiated and adds no"] # [doc = " benefit as opposed to tuple initializers"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " struct TupleStruct(u8, u16);"] # [doc = ""] # [doc = " let _ = TupleStruct {"] # [doc = "     0: 1,"] # [doc = "     1: 23,"] # [doc = " };"] # [doc = ""] # [doc = " // should be written as"] # [doc = " let base = TupleStruct(1, 23);"] # [doc = ""] # [doc = " // This is OK however"] # [doc = " let _ = TupleStruct { 0: 42, ..base };"] # [doc = " ```"] # [clippy :: version = "1.59.0"] pub INIT_NUMBERED_FIELDS , style , "numbered fields in tuple struct initializer" }
};
}
