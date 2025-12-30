// Generated macro for impl_11 (impl)
macro_rules! Depcrateimpl_11 {
() => {
// Module: crate
// Provides: {"impl_11"}
// Dependencies: {}
impl < 'a > MockIo < 'a > { fn next_byte (& mut self) -> Option < u8 > { if let Some (& c) = self . input . first () { self . input = & self . input [1 ..] ; Some (c) } else { None } } fn next_u32 (& mut self) -> u32 { (self . next_byte () . unwrap_or (0) as u32) << 8 | self . next_byte () . unwrap_or (0) as u32 } }
};
}
