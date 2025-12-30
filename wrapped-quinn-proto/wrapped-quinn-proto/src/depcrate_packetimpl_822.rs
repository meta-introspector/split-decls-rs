// Generated macro for impl_822 (impl)
macro_rules! Depcrate_packetimpl_822 {
() => {
// Module: crate::packet
// Provides: {"impl_822"}
// Dependencies: {}
impl Packet { pub (crate) fn reserved_bits_valid (& self) -> bool { let mask = match self . header { Header :: Short { .. } => SHORT_RESERVED_BITS , _ => LONG_RESERVED_BITS , } ; self . header_data [0] & mask == 0 } }
};
}
