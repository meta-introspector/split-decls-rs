// Generated macro for impl_9 (impl)
macro_rules! Depcrateimpl_9 {
() => {
// Module: crate
// Provides: {"impl_9"}
// Dependencies: {}
impl < T : Deref > COption < T > { # [doc = " Converts from `COption<T>` (or `&COption<T>`) to `COption<&T::Target>`."] # [doc = ""] # [doc = " Leaves the original COption in-place, creating a new one with a reference"] # [doc = " to the original one, additionally coercing the contents via [`Deref`]."] # [doc = ""] # [doc = " [`Deref`]: ../../std/ops/trait.Deref.html"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```ignore"] # [doc = " #![feature(inner_deref)]"] # [doc = ""] # [doc = " let x: COption<String> = COption::Some(\"hey\".to_owned());"] # [doc = " assert_eq!(x.as_deref(), COption::Some(\"hey\"));"] # [doc = ""] # [doc = " let x: COption<String> = COption::None;"] # [doc = " assert_eq!(x.as_deref(), COption::None);"] # [doc = " ```"] pub fn as_deref (& self) -> COption < & T :: Target > { self . as_ref () . map (| t | t . deref ()) } }
};
}
