// Generated macro for macro_373 (macro)
macro_rules! Depcrate_arbitrary__alloc_strmacro_373 {
() => {
// Module: crate::arbitrary::_alloc::str
// Provides: {"macro_373"}
// Dependencies: {}
arbitrary ! (Utf8Error , SFnPtrMap < (StrategyFor < u16 >, ELSeqs) , Utf8Error >; static_map ((any ::< u16 > () , gen_el_seqs ()) , | (vut , elseq) | { let v = repeat (b'_') . take (vut as usize) . chain (elseq . iter () . cloned ()) . collect ::< Vec < u8 >> () ; from_utf8 (& v) . unwrap_err () })) ;
};
}
