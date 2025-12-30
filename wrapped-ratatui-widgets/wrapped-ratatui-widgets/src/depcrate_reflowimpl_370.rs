// Generated macro for impl_370 (impl)
macro_rules! Depcrate_reflowimpl_370 {
() => {
// Module: crate::reflow
// Provides: {"impl_370"}
// Dependencies: {}
impl < 'a , O , I > LineComposer < 'a > for WordWrapper < 'a , O , I > where O : Iterator < Item = (I , Alignment) > , I : Iterator < Item = StyledGrapheme < 'a > > , { fn next_line < 'lend > (& 'lend mut self) -> Option < WrappedLine < 'lend , 'a > > { if self . max_line_width == 0 { return None ; } loop { if let Some (line) = self . wrapped_lines . pop_front () { let line_width = line . iter () . map (| grapheme | grapheme . symbol . width () as u16) . sum () ; self . replace_current_line (line) ; return Some (WrappedLine { graphemes : & self . current_line , width : line_width , alignment : self . current_alignment , }) ; } let (line_symbols , line_alignment) = self . input_lines . next () ? ; self . current_alignment = line_alignment ; self . process_input (line_symbols) ; } } }
};
}
