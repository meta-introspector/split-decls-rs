// Generated macro for impl_5 (impl)
macro_rules! Depcrateimpl_5 {
() => {
// Module: crate
// Provides: {"impl_5"}
// Dependencies: {}
impl < T : Copy > COption < & mut T > { # [doc = " Maps an `COption<&mut T>` to an `COption<T>` by copying the contents of the"] # [doc = " option."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```ignore"] # [doc = " let mut x = 12;"] # [doc = " let opt_x = COption::Some(&mut x);"] # [doc = " assert_eq!(opt_x, COption::Some(&mut 12));"] # [doc = " let copied = opt_x.copied();"] # [doc = " assert_eq!(copied, COption::Some(12));"] # [doc = " ```"] pub fn copied (self) -> COption < T > { self . map (| & mut t | t) } }
};
}
