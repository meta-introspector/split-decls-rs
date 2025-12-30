// Generated macro for ToStr (trait)
macro_rules! DepcrateToStr {
() => {
// Module: crate
// Provides: {"ToStr"}
// Dependencies: {}
# [doc = " A trait for converting a program's specific error type to a `&str`."] # [doc = ""] # [doc = " Can be used with `ProgramError::to_str::<E>()` to get an error string"] # [doc = " belonging to a specific program's error if the variant is"] # [doc = " `ProgramError::Custom(...)`, or generic strings from the contained"] # [doc = " `ProgramError` for all other variants."] # [doc = ""] # [doc = " The `ProgramError::to_str::<E>()` function also requires implementing"] # [doc = " `TryFrom<u32>` on an error type, which can be done easily using"] # [doc = " `num_enum::TryFromPrimitive`."] pub trait ToStr { fn to_str (& self) -> & 'static str ; }
};
}
