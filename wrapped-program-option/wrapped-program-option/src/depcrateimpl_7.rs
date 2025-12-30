// Generated macro for impl_7 (impl)
macro_rules! Depcrateimpl_7 {
() => {
// Module: crate
// Provides: {"impl_7"}
// Dependencies: {}
impl < T : Clone > COption < & mut T > { # [doc = " Maps an `COption<&mut T>` to an `COption<T>` by cloning the contents of the"] # [doc = " option."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```ignore"] # [doc = " let mut x = 12;"] # [doc = " let opt_x = COption::Some(&mut x);"] # [doc = " assert_eq!(opt_x, COption::Some(&mut 12));"] # [doc = " let cloned = opt_x.cloned();"] # [doc = " assert_eq!(cloned, COption::Some(12));"] # [doc = " ```"] pub fn cloned (self) -> COption < T > { self . map (| t | t . clone ()) } }
};
}
