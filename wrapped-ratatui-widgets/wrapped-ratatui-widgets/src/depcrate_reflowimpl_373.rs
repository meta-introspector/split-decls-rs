// Generated macro for impl_373 (impl)
macro_rules! Depcrate_reflowimpl_373 {
() => {
// Module: crate::reflow
// Provides: {"impl_373"}
// Dependencies: {}
impl < 'a , O , I > LineComposer < 'a > for LineTruncator < 'a , O , I > where O : Iterator < Item = (I , Alignment) > , I : Iterator < Item = StyledGrapheme < 'a > > , { fn next_line < 'lend > (& 'lend mut self) -> Option < WrappedLine < 'lend , 'a > > { if self . max_line_width == 0 { return None ; } self . current_line . truncate (0) ; let mut current_line_width = 0 ; let mut lines_exhausted = true ; let mut horizontal_offset = self . horizontal_offset as usize ; let mut current_alignment = Alignment :: Left ; if let Some ((current_line , alignment)) = & mut self . input_lines . next () { lines_exhausted = false ; current_alignment = * alignment ; for StyledGrapheme { symbol , style } in current_line { if symbol . width () as u16 > self . max_line_width { continue ; } if current_line_width + symbol . width () as u16 > self . max_line_width { break ; } let symbol = if horizontal_offset == 0 || Alignment :: Left != * alignment { symbol } else { let w = symbol . width () ; if w > horizontal_offset { let t = trim_offset (symbol , horizontal_offset) ; horizontal_offset = 0 ; t } else { horizontal_offset -= w ; "" } } ; current_line_width += symbol . width () as u16 ; self . current_line . push (StyledGrapheme { symbol , style }) ; } } if lines_exhausted { None } else { Some (WrappedLine { graphemes : & self . current_line , width : current_line_width , alignment : current_alignment , }) } } }
};
}
