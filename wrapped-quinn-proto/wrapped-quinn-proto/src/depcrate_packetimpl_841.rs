// Generated macro for impl_841 (impl)
macro_rules! Depcrate_packetimpl_841 {
() => {
// Module: crate::packet
// Provides: {"impl_841"}
// Dependencies: {}
impl From < LongHeaderType > for u8 { fn from (ty : LongHeaderType) -> Self { use { LongHeaderType :: * , LongType :: * } ; match ty { Initial => LONG_HEADER_FORM | FIXED_BIT , Standard (ZeroRtt) => LONG_HEADER_FORM | FIXED_BIT | (0x1 << 4) , Standard (Handshake) => LONG_HEADER_FORM | FIXED_BIT | (0x2 << 4) , Retry => LONG_HEADER_FORM | FIXED_BIT | (0x3 << 4) , } } }
};
}
