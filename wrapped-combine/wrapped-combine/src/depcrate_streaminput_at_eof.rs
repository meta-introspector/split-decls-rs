// Generated macro for input_at_eof (function)
macro_rules! Depcrate_streaminput_at_eof {
() => {
// Module: crate::stream
// Provides: {"input_at_eof"}
// Dependencies: {}
# [doc (hidden)] pub fn input_at_eof < Input > (input : & mut Input) -> bool where Input : ? Sized + Stream , { let before = input . checkpoint () ; let x = input . uncons () . err () . map_or (false , | err | err . is_unexpected_end_of_input ()) ; input . reset (before) . is_ok () && x }
};
}
