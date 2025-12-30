// Generated macro for impl_28 (impl)
macro_rules! Depcrateimpl_28 {
() => {
// Module: crate
// Provides: {"impl_28"}
// Dependencies: {}
impl < 'a > HexSlice < 'a > { pub fn new < T > (data : & 'a T) -> HexSlice < 'a > where T : ? Sized + AsRef < [u8] > + 'a , { HexSlice (data . as_ref ()) } pub fn maybe_string < T > (data : Option < & 'a T >) -> Option < String > where T : ? Sized + AsRef < [u8] > + 'a , { data . map (| d | format ! ("{}" , HexSlice :: new (d))) } }
};
}
