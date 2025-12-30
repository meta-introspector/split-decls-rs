// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
impl < T > COption < COption < T > > { # [doc = " Converts from `COption<COption<T>>` to `COption<T>`"] # [doc = ""] # [doc = " # Examples"] # [doc = " Basic usage:"] # [doc = " ```ignore"] # [doc = " #![feature(option_flattening)]"] # [doc = " let x: COption<COption<u32>> = COption::Some(COption::Some(6));"] # [doc = " assert_eq!(COption::Some(6), x.flatten());"] # [doc = ""] # [doc = " let x: COption<COption<u32>> = COption::Some(COption::None);"] # [doc = " assert_eq!(COption::None, x.flatten());"] # [doc = ""] # [doc = " let x: COption<COption<u32>> = COption::None;"] # [doc = " assert_eq!(COption::None, x.flatten());"] # [doc = " ```"] # [doc = " Flattening once only removes one level of nesting:"] # [doc = " ```ignore"] # [doc = " #![feature(option_flattening)]"] # [doc = " let x: COption<COption<COption<u32>>> = COption::Some(COption::Some(COption::Some(6)));"] # [doc = " assert_eq!(COption::Some(COption::Some(6)), x.flatten());"] # [doc = " assert_eq!(COption::Some(6), x.flatten().flatten());"] # [doc = " ```"] # [inline] pub fn flatten (self) -> COption < T > { self . and_then (convert :: identity) } }
};
}
