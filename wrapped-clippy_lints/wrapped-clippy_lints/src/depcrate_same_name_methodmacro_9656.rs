// Generated macro for macro_9656 (macro)
macro_rules! Depcrate_same_name_methodmacro_9656 {
() => {
// Module: crate::same_name_method
// Provides: {"macro_9656"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " It lints if a struct has two methods with the same name:"] # [doc = " one from a trait, another not from a trait."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Confusing."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " trait T {"] # [doc = "     fn foo(&self) {}"] # [doc = " }"] # [doc = ""] # [doc = " struct S;"] # [doc = ""] # [doc = " impl T for S {"] # [doc = "     fn foo(&self) {}"] # [doc = " }"] # [doc = ""] # [doc = " impl S {"] # [doc = "     fn foo(&self) {}"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.57.0"] pub SAME_NAME_METHOD , restriction , "two method with same name" }
};
}
