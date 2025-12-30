// Generated macro for GRAPHEME_BREAK_REV (static)
macro_rules! Depcrate_unicode_fsm_grapheme_break_revGRAPHEME_BREAK_REV {
() => {
// Module: crate::unicode::fsm::grapheme_break_rev
// Provides: {"GRAPHEME_BREAK_REV"}
// Dependencies: {}
pub static GRAPHEME_BREAK_REV : Lazy < DFA < & 'static [u8] > > = Lazy :: new (| | { # [cfg (target_endian = "big")] static BYTES : & 'static [u8] = include_bytes ! ("grapheme_break_rev.bigendian.dfa") ; # [cfg (target_endian = "little")] static BYTES : & 'static [u8] = include_bytes ! ("grapheme_break_rev.littleendian.dfa") ; let (dfa , _) = DFA :: from_bytes (BYTES) . expect ("serialized DFA should be valid") ; dfa }) ;
};
}
