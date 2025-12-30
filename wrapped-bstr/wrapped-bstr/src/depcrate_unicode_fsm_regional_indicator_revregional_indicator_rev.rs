// Generated macro for REGIONAL_INDICATOR_REV (static)
macro_rules! Depcrate_unicode_fsm_regional_indicator_revREGIONAL_INDICATOR_REV {
() => {
// Module: crate::unicode::fsm::regional_indicator_rev
// Provides: {"REGIONAL_INDICATOR_REV"}
// Dependencies: {}
pub static REGIONAL_INDICATOR_REV : Lazy < DFA < & 'static [u32] > > = Lazy :: new (| | { static ALIGNED : & AlignAs < [u8] , u32 > = & AlignAs { _align : [] , # [cfg (target_endian = "big")] bytes : * include_bytes ! ("regional_indicator_rev.bigendian.dfa") , # [cfg (target_endian = "little")] bytes : * include_bytes ! ("regional_indicator_rev.littleendian.dfa") , } ; let (dfa , _) = DFA :: from_bytes (& ALIGNED . bytes) . expect ("serialized DFA should be valid") ; dfa }) ;
};
}
