// Generated macro for impl_6 (impl)
macro_rules! Depcrateimpl_6 {
() => {
// Module: crate
// Provides: {"impl_6"}
// Dependencies: {}
impl < T : Clone > COption < & T > { # [doc = " Maps an `COption<&T>` to an `COption<T>` by cloning the contents of the"] # [doc = " option."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```ignore"] # [doc = " let x = 12;"] # [doc = " let opt_x = COption::Some(&x);"] # [doc = " assert_eq!(opt_x, COption::Some(&12));"] # [doc = " let cloned = opt_x.cloned();"] # [doc = " assert_eq!(cloned, COption::Some(12));"] # [doc = " ```"] pub fn cloned (self) -> COption < T > { self . map (| t | t . clone ()) } }
};
}
