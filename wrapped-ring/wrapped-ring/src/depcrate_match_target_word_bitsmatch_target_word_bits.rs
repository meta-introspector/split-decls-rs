// Generated macro for match_target_word_bits (macro)
macro_rules! Depcrate_match_target_word_bitsmatch_target_word_bits {
() => {
// Module: crate::match_target_word_bits
// Provides: {"match_target_word_bits"}
// Dependencies: {}
macro_rules ! match_target_word_bits { { 64 => { $ ($ if_64 : tt) * } , 32 => { $ ($ if_32 : tt) * } , $ (_ => { $ ($ otherwise : tt) * }) ? } => { cfg_if :: cfg_if ! { if # [cfg (any (target_arch = "aarch64" , target_arch = "x86_64" , target_pointer_width = "64"))] { $ ($ if_64) * } else if # [cfg (target_pointer_width = "32")] { $ ($ if_32) * } else { $ ($ ($ otherwise) *) ? } } } ; { 64 | 32 => { $ ($ if_64_or_32 : item) * } , $ (_ => { $ ($ otherwise : tt) * }) ? } => { cfg_if :: cfg_if ! { if # [cfg (any (target_pointer_width = "32" , target_pointer_width = "64"))] { $ ($ if_64_or_32) * } else { $ ($ ($ otherwise) *) ? } } } ; }
};
}
