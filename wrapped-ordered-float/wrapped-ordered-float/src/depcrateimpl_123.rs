// Generated macro for impl_123 (impl)
macro_rules! Depcrateimpl_123 {
() => {
// Module: crate
// Provides: {"impl_123"}
// Dependencies: {}
impl < T : FloatCore + Num > Num for OrderedFloat < T > { type FromStrRadixErr = T :: FromStrRadixErr ; fn from_str_radix (str : & str , radix : u32) -> Result < Self , Self :: FromStrRadixErr > { T :: from_str_radix (str , radix) . map (OrderedFloat) } }
};
}
