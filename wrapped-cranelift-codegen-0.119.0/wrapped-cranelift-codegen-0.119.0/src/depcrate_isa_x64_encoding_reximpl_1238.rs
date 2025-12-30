// Generated macro for impl_1238 (impl)
macro_rules! Depcrate_isa_x64_encoding_reximpl_1238 {
() => {
// Module: crate::isa::x64::encoding::rex
// Provides: {"impl_1238"}
// Dependencies: {}
impl LegacyPrefixes { # [doc = " Emit the legacy prefix as bytes (e.g. in REX instructions)."] # [inline (always)] pub (crate) fn emit < BS : ByteSink + ? Sized > (& self , sink : & mut BS) { match self { Self :: _66 => sink . put1 (0x66) , Self :: _F0 => sink . put1 (0xF0) , Self :: _66F0 => { sink . put1 (0x66) ; sink . put1 (0xF0) ; } Self :: _F2 => sink . put1 (0xF2) , Self :: _F3 => sink . put1 (0xF3) , Self :: _66F3 => { sink . put1 (0x66) ; sink . put1 (0xF3) ; } Self :: None => () , } } # [doc = " Emit the legacy prefix as bits (e.g. for EVEX instructions)."] # [inline (always)] pub (crate) fn bits (& self) -> u8 { match self { Self :: None => 0b00 , Self :: _66 => 0b01 , Self :: _F3 => 0b10 , Self :: _F2 => 0b11 , _ => panic ! ("VEX and EVEX bits can only be extracted from single prefixes: None, 66, F3, F2") , } } }
};
}
