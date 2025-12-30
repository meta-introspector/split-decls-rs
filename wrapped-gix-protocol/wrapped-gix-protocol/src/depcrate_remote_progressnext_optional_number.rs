// Generated macro for next_optional_number (function)
macro_rules! Depcrate_remote_progressnext_optional_number {
() => {
// Module: crate::remote_progress
// Provides: {"next_optional_number"}
// Dependencies: {}
fn next_optional_number (i : & mut & [u8]) -> ModalResult < Option < usize > , () > { opt (preceded (take_till (0 .. , | c : u8 | c . is_ascii_digit ()) , parse_number)) . parse_next (i) }
};
}
