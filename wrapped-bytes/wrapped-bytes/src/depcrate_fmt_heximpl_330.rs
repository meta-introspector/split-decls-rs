// Generated macro for impl_330 (impl)
macro_rules! Depcrate_fmt_heximpl_330 {
() => {
// Module: crate::fmt::hex
// Provides: {"impl_330"}
// Dependencies: {}
impl LowerHex for BytesRef < '_ > { fn fmt (& self , f : & mut Formatter < '_ >) -> Result { for & b in self . 0 { write ! (f , "{:02x}" , b) ? ; } Ok (()) } }
};
}
