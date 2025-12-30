// Generated macro for impl_914 (impl)
macro_rules! Depcrate_outputimpl_914 {
() => {
// Module: crate::output
// Provides: {"impl_914"}
// Dependencies: {}
impl TerminalWidth { pub fn actual_terminal_width (self) -> Option < usize > { # [cfg (unix)] let stdout_term_width = { terminal_size :: terminal_size_of (std :: io :: stdout ()) . map (| (w , _h) | w . 0 as _) } ; # [cfg (windows)] let stdout_term_width = { use std :: os :: windows :: io :: BorrowedHandle ; use windows_sys :: Win32 :: System :: Console :: { GetStdHandle , STD_OUTPUT_HANDLE } ; terminal_size :: terminal_size_of (unsafe { BorrowedHandle :: borrow_raw (GetStdHandle (STD_OUTPUT_HANDLE)) }) . map (| (w , _h) | w . 0 as _) } ; # [rustfmt :: skip] return match self { Self :: Set (width) => Some (width) , Self :: Automatic => stdout_term_width , } ; } }
};
}
