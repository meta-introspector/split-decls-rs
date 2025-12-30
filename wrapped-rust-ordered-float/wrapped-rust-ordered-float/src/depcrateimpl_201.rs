// Generated macro for impl_201 (impl)
macro_rules! Depcrateimpl_201 {
() => {
// Module: crate
// Provides: {"impl_201"}
// Dependencies: {}
impl < T : FloatCore > Num for NotNan < T > { type FromStrRadixErr = ParseNotNanError < T :: FromStrRadixErr > ; fn from_str_radix (src : & str , radix : u32) -> Result < Self , Self :: FromStrRadixErr > { T :: from_str_radix (src , radix) . map_err (ParseNotNanError :: ParseFloatError) . and_then (| n | NotNan :: new (n) . map_err (| _ | ParseNotNanError :: IsNaN)) } }
};
}
