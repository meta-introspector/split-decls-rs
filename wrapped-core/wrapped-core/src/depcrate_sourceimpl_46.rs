// Generated macro for impl_46 (impl)
macro_rules! Depcrate_sourceimpl_46 {
() => {
// Module: crate::source
// Provides: {"impl_46"}
// Dependencies: {}
impl Source { pub fn append_src (& mut self , src : & Source) { self . s . push_str (& src . s) ; self . indent += src . indent ; self . in_line_comment = src . in_line_comment ; } pub fn push_str (& mut self , src : & str) { let lines = src . lines () . collect :: < Vec < _ > > () ; for (i , line) in lines . iter () . enumerate () { if ! self . continuing_line { if ! line . is_empty () { for _ in 0 .. self . indent { self . s . push_str ("  ") ; } } self . continuing_line = true ; } let trimmed = line . trim () ; if trimmed . starts_with ("//") { self . in_line_comment = true ; } if ! self . in_line_comment { if trimmed . starts_with ('}') && self . s . ends_with ("  ") { self . s . pop () ; self . s . pop () ; } } self . s . push_str (if lines . len () == 1 { line } else { line . trim_start () }) ; if ! self . in_line_comment { if trimmed . ends_with ('{') { self . indent += 1 ; } if trimmed . starts_with ('}') { self . indent = self . indent . saturating_sub (1) ; } } if i != lines . len () - 1 || src . ends_with ('\n') { self . newline () ; } } } pub fn indent (& mut self , amt : usize) { self . indent += amt ; } pub fn deindent (& mut self , amt : usize) { self . indent -= amt ; } # [doc = " Set the indentation level, and return the old level."] pub fn set_indent (& mut self , amt : usize) -> usize { let old = self . indent ; self . indent = amt ; old } fn newline (& mut self) { self . in_line_comment = false ; self . continuing_line = false ; self . s . push ('\n') ; } pub fn as_mut_string (& mut self) -> & mut String { & mut self . s } pub fn as_str (& self) -> & str { & self . s } }
};
}
