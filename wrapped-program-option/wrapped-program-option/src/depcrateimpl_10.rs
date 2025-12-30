// Generated macro for impl_10 (impl)
macro_rules! Depcrateimpl_10 {
() => {
// Module: crate
// Provides: {"impl_10"}
// Dependencies: {}
impl < T : DerefMut > COption < T > { # [doc = " Converts from `COption<T>` (or `&mut COption<T>`) to `COption<&mut T::Target>`."] # [doc = ""] # [doc = " Leaves the original `COption` in-place, creating a new one containing a mutable reference to"] # [doc = " the inner type's `Deref::Target` type."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```ignore"] # [doc = " #![feature(inner_deref)]"] # [doc = ""] # [doc = " let mut x: COption<String> = COption::Some(\"hey\".to_owned());"] # [doc = " assert_eq!(x.as_deref_mut().map(|x| {"] # [doc = "     x.make_ascii_uppercase();"] # [doc = "     x"] # [doc = " }), COption::Some(\"HEY\".to_owned().as_mut_str()));"] # [doc = " ```"] pub fn as_deref_mut (& mut self) -> COption < & mut T :: Target > { self . as_mut () . map (| t | t . deref_mut ()) } }
};
}
