// Generated macro for impl_31 (impl)
macro_rules! Depcrateimpl_31 {
() => {
// Module: crate
// Provides: {"impl_31"}
// Dependencies: {}
impl Iterator for PercentDecode < '_ > { type Item = u8 ; fn next (& mut self) -> Option < u8 > { self . bytes . next () . map (| & byte | { if byte == b'%' { after_percent_sign (& mut self . bytes) . unwrap_or (byte) } else { byte } }) } fn size_hint (& self) -> (usize , Option < usize >) { let bytes = self . bytes . len () ; ((bytes + 2) / 3 , Some (bytes)) } }
};
}
