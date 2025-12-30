// Generated macro for impl_331 (impl)
macro_rules! Depcrate_fmt_heximpl_331 {
() => {
// Module: crate::fmt::hex
// Provides: {"impl_331"}
// Dependencies: {}
impl UpperHex for BytesRef < '_ > { fn fmt (& self , f : & mut Formatter < '_ >) -> Result { for & b in self . 0 { write ! (f , "{:02X}" , b) ? ; } Ok (()) } }
};
}
