// Generated macro for gen_el_seqs (function)
macro_rules! Depcrate_arbitrary__alloc_strgen_el_seqs {
() => {
// Module: crate::arbitrary::_alloc::str
// Provides: {"gen_el_seqs"}
// Dependencies: {}
fn gen_el_seqs () -> ELSeqs { prop_oneof ! [Just (& [0xC2]) , Just (& [0x80]) , Just (& [0xE0 , 0xA0 , 0x00]) , Just (& [0xF0 , 0x90 , 0x80 , 0x00])] }
};
}
