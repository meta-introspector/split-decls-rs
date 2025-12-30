// Generated macro for impl_167 (impl)
macro_rules! Depcrate_gv_recordimpl_167 {
() => {
// Module: crate::gv::record
// Provides: {"impl_167"}
// Dependencies: {}
impl RecordParser { pub fn new (input : & str) -> Self { Self { input : input . chars () . collect () , pos : 0 , } } pub fn parse (& mut self) -> RecordDef { let mut frame = RecordParserFrame :: new () ; loop { let ch = self . input [self . pos] ; match ch { '{' => { self . pos += 1 ; frame . finalize_label () ; let ret = self . parse () ; frame . arr . push (ret) ; } '|' => { self . pos += 1 ; frame . finalize_label () ; } '}' => { self . pos += 1 ; frame . finalize_label () ; return frame . finalize_record () ; } _ => { self . pos += 1 ; frame . label . push (ch) ; } } if self . pos == self . input . len () { return frame . finalize_record () ; } } } }
};
}
