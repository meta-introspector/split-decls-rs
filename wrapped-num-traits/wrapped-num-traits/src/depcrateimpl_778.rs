// Generated macro for impl_778 (impl)
macro_rules! Depcrateimpl_778 {
() => {
// Module: crate
// Provides: {"impl_778"}
// Dependencies: {}
impl < T : Num > Num for Wrapping < T > where Wrapping < T > : NumOps , { type FromStrRadixErr = T :: FromStrRadixErr ; fn from_str_radix (str : & str , radix : u32) -> Result < Self , Self :: FromStrRadixErr > { T :: from_str_radix (str , radix) . map (Wrapping) } }
};
}
