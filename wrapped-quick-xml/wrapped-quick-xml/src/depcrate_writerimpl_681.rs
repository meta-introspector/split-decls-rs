// Generated macro for impl_681 (impl)
macro_rules! Depcrate_writerimpl_681 {
() => {
// Module: crate::writer
// Provides: {"impl_681"}
// Dependencies: {}
impl Indentation { pub fn new (indent_char : u8 , indent_size : usize) -> Self { Self { should_line_break : false , indent_char , indent_size , indents : vec ! [indent_char ; 128] , current_indent_len : 0 , } } # [doc = " Increase indentation by one level"] pub fn grow (& mut self) { self . current_indent_len += self . indent_size ; self . ensure (self . current_indent_len) ; } # [doc = " Decrease indentation by one level. Do nothing, if level already zero"] pub fn shrink (& mut self) { self . current_indent_len = self . current_indent_len . saturating_sub (self . indent_size) ; } # [doc = " Returns indent string for current level"] pub fn current (& self) -> & [u8] { & self . indents [.. self . current_indent_len] } # [doc = " Returns indent with current indent plus additional indent"] pub fn additional (& mut self , additional_indent : usize) -> & [u8] { let new_len = self . current_indent_len + additional_indent ; self . ensure (new_len) ; & self . indents [.. new_len] } fn ensure (& mut self , new_len : usize) { if self . indents . len () < new_len { self . indents . resize (new_len , self . indent_char) ; } } }
};
}
