// Generated macro for SIMPLE_WORD_FWD (static)
macro_rules! Depcrate_unicode_fsm_simple_word_fwdSIMPLE_WORD_FWD {
() => {
// Module: crate::unicode::fsm::simple_word_fwd
// Provides: {"SIMPLE_WORD_FWD"}
// Dependencies: {}
pub static SIMPLE_WORD_FWD : Lazy < DFA < & 'static [u8] > > = Lazy :: new (| | { # [cfg (target_endian = "big")] static BYTES : & 'static [u8] = include_bytes ! ("simple_word_fwd.bigendian.dfa") ; # [cfg (target_endian = "little")] static BYTES : & 'static [u8] = include_bytes ! ("simple_word_fwd.littleendian.dfa") ; let (dfa , _) = DFA :: from_bytes (BYTES) . expect ("serialized DFA should be valid") ; dfa }) ;
};
}
