// Generated macro for WHITESPACE_ANCHORED_FWD (static)
macro_rules! Depcrate_unicode_fsm_whitespace_anchored_fwdWHITESPACE_ANCHORED_FWD {
() => {
// Module: crate::unicode::fsm::whitespace_anchored_fwd
// Provides: {"WHITESPACE_ANCHORED_FWD"}
// Dependencies: {}
pub static WHITESPACE_ANCHORED_FWD : Lazy < DFA < & 'static [u32] > > = Lazy :: new (| | { static ALIGNED : & AlignAs < [u8] , u32 > = & AlignAs { _align : [] , # [cfg (target_endian = "big")] bytes : * include_bytes ! ("whitespace_anchored_fwd.bigendian.dfa") , # [cfg (target_endian = "little")] bytes : * include_bytes ! ("whitespace_anchored_fwd.littleendian.dfa") , } ; let (dfa , _) = DFA :: from_bytes (& ALIGNED . bytes) . expect ("serialized DFA should be valid") ; dfa }) ;
};
}
