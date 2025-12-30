// Generated macro for macro_3006 (macro)
macro_rules! Depcrate_infallible_try_frommacro_3006 {
() => {
// Module: crate::infallible_try_from
// Provides: {"macro_3006"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = ""] # [doc = " Finds manual impls of `TryFrom` with infallible error types."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = ""] # [doc = " Infallible conversions should be implemented via `From` with the blanket conversion."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " use std::convert::Infallible;"] # [doc = " struct MyStruct(i16);"] # [doc = " impl TryFrom<i16> for MyStruct {"] # [doc = "     type Error = Infallible;"] # [doc = "     fn try_from(other: i16) -> Result<Self, Infallible> {"] # [doc = "         Ok(Self(other.into()))"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " struct MyStruct(i16);"] # [doc = " impl From<i16> for MyStruct {"] # [doc = "     fn from(other: i16) -> Self {"] # [doc = "         Self(other)"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.89.0"] pub INFALLIBLE_TRY_FROM , suspicious , "TryFrom with infallible Error type" }
};
}
