// Generated macro for next_optional_percentage (function)
macro_rules! Depcrate_remote_progressnext_optional_percentage {
() => {
// Module: crate::remote_progress
// Provides: {"next_optional_percentage"}
// Dependencies: {}
fn next_optional_percentage (i : & mut & [u8]) -> ModalResult < Option < u32 > , () > { opt (terminated (preceded (take_till (0 .. , | c : u8 | c . is_ascii_digit ()) , parse_number . try_map (u32 :: try_from) ,) , b"%" ,)) . parse_next (i) }
};
}
