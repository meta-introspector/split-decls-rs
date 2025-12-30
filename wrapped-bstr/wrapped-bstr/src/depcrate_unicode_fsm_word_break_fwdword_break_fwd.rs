// Generated macro for WORD_BREAK_FWD (static)
macro_rules! Depcrate_unicode_fsm_word_break_fwdWORD_BREAK_FWD {
() => {
// Module: crate::unicode::fsm::word_break_fwd
// Provides: {"WORD_BREAK_FWD"}
// Dependencies: {}
pub static WORD_BREAK_FWD : Lazy < DFA < & 'static [u8] > > = Lazy :: new (| | { # [cfg (target_endian = "big")] static BYTES : & 'static [u8] = include_bytes ! ("word_break_fwd.bigendian.dfa") ; # [cfg (target_endian = "little")] static BYTES : & 'static [u8] = include_bytes ! ("word_break_fwd.littleendian.dfa") ; let (dfa , _) = DFA :: from_bytes (BYTES) . expect ("serialized DFA should be valid") ; dfa }) ;
};
}
