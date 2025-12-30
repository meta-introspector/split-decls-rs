// Generated macro for macro_1345 (macro)
macro_rules! Depcrate_default_constructed_unit_structsmacro_1345 {
() => {
// Module: crate::default_constructed_unit_structs
// Provides: {"macro_1345"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for construction on unit struct using `default`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This adds code complexity and an unnecessary function call."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # use std::marker::PhantomData;"] # [doc = " #[derive(Default)]"] # [doc = " struct S<T> {"] # [doc = "     _marker: PhantomData<T>"] # [doc = " }"] # [doc = ""] # [doc = " let _: S<i32> = S {"] # [doc = "     _marker: PhantomData::default()"] # [doc = " };"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # use std::marker::PhantomData;"] # [doc = " struct S<T> {"] # [doc = "     _marker: PhantomData<T>"] # [doc = " }"] # [doc = ""] # [doc = " let _: S<i32> = S {"] # [doc = "     _marker: PhantomData"] # [doc = " };"] # [doc = " ```"] # [clippy :: version = "1.71.0"] pub DEFAULT_CONSTRUCTED_UNIT_STRUCTS , complexity , "unit structs can be constructed without calling `default`" }
};
}
