// Generated macro for impl_92 (impl)
macro_rules! Depcrate_serializerimpl_92 {
() => {
// Module: crate::serializer
// Provides: {"impl_92"}
// Dependencies: {}
impl TextWriter { fn indent (& mut self) { self . indent_level += 1 ; } fn dedent (& mut self) { self . indent_level = self . indent_level . checked_sub (1) . expect ("Dedenting without a corresponding indent") ; } fn write_indent (& mut self) { for _ in 0 .. self . indent_level { self . buffer . push_str ("    ") ; } } fn newline (& mut self) { if self . buffer . ends_with ('\r') { self . buffer . push ('\r') ; } self . buffer . push ('\n') ; } fn write_literal (& mut self , item : & str) { if self . buffer . ends_with ('\n') { self . write_indent () ; } write ! (self . buffer , "{}" , item) . expect ("Writing to an in-memory buffer never fails") ; } fn write_char_into_indent (& mut self , ch : char) { if self . buffer . ends_with ('\n') { self . write_indent () ; } self . buffer . pop () ; self . buffer . push (ch) ; } }
};
}
