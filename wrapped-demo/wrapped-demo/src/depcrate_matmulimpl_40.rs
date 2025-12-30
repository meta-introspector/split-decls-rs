// Generated macro for impl_40 (impl)
macro_rules! Depcrate_matmulimpl_40 {
() => {
// Module: crate::matmul
// Provides: {"impl_40"}
// Dependencies: {}
impl Iterator for SplayedBitsCounter { type Item = usize ; fn next (& mut self) -> Option < usize > { let prev = self . value & 0x5555_5555 ; if prev < self . max { self . value |= 0xaaaa_aaaa ; self . value += 1 ; Some (prev) } else { None } } }
};
}
