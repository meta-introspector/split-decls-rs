// Generated macro for parse_number (function)
macro_rules! Depcrate_remote_progressparse_number {
() => {
// Module: crate::remote_progress
// Provides: {"parse_number"}
// Dependencies: {}
fn parse_number (i : & mut & [u8]) -> ModalResult < usize , () > { take_till (0 .. , | c : u8 | ! c . is_ascii_digit ()) . try_map (gix_utils :: btoi :: to_signed) . parse_next (i) }
};
}
