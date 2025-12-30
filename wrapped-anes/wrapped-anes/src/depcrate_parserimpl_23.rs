// Generated macro for impl_23 (impl)
macro_rules! Depcrate_parserimpl_23 {
() => {
// Module: crate::parser
// Provides: {"impl_23"}
// Dependencies: {}
impl Provide for SequenceProvider { fn provide_char (& mut self , ch : char) { if let Some (seq) = parsers :: parse_char (ch , self . esc_o) { self . seqs . push_back (seq) ; } self . esc_o = false ; } fn provide_esc_sequence (& mut self , ch : char) { if ch == 'O' { self . esc_o = true ; } else { self . esc_o = false ; if let Some (seq) = parsers :: parse_esc_sequence (ch) { self . seqs . push_back (seq) ; } } } fn provide_csi_sequence (& mut self , parameters : & [u64] , ignored_count : usize , ch : char) { if let Some (seq) = parsers :: parse_csi_sequence (parameters , ignored_count , ch) { self . seqs . push_back (seq) ; } self . esc_o = false ; } }
};
}
