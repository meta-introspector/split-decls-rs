// Generated macro for macro_1528 (macro)
macro_rules! Depcrate_derivemacro_1528 {
() => {
// Module: crate::derive
// Provides: {"macro_1528"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for types that derive `PartialEq` and could implement `Eq`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " If a type `T` derives `PartialEq` and all of its members implement `Eq`,"] # [doc = " then `T` can always implement `Eq`. Implementing `Eq` allows `T` to be used"] # [doc = " in APIs that require `Eq` types. It also allows structs containing `T` to derive"] # [doc = " `Eq` themselves."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " #[derive(PartialEq)]"] # [doc = " struct Foo {"] # [doc = "     i_am_eq: i32,"] # [doc = "     i_am_eq_too: Vec<String>,"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " #[derive(PartialEq, Eq)]"] # [doc = " struct Foo {"] # [doc = "     i_am_eq: i32,"] # [doc = "     i_am_eq_too: Vec<String>,"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.63.0"] pub DERIVE_PARTIAL_EQ_WITHOUT_EQ , nursery , "deriving `PartialEq` on a type that can implement `Eq`, without implementing `Eq`" }
};
}
