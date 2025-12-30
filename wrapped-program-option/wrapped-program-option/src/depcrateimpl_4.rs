// Generated macro for impl_4 (impl)
macro_rules! Depcrateimpl_4 {
() => {
// Module: crate
// Provides: {"impl_4"}
// Dependencies: {}
impl < T : Copy > COption < & T > { # [doc = " Maps an `COption<&T>` to an `COption<T>` by copying the contents of the"] # [doc = " option."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```ignore"] # [doc = " let x = 12;"] # [doc = " let opt_x = COption::Some(&x);"] # [doc = " assert_eq!(opt_x, COption::Some(&12));"] # [doc = " let copied = opt_x.copied();"] # [doc = " assert_eq!(copied, COption::Some(12));"] # [doc = " ```"] pub fn copied (self) -> COption < T > { self . map (| & t | t) } }
};
}
