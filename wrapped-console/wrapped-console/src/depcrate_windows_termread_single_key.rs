// Generated macro for read_single_key (function)
macro_rules! Depcrate_windows_termread_single_key {
() => {
// Module: crate::windows_term
// Provides: {"read_single_key"}
// Dependencies: {}
pub (crate) fn read_single_key (ctrlc_key : bool) -> io :: Result < Key > { let key_event = { let _guard = ctrlc_key . then (| | { ConsoleModeGuard :: set (unsafe { GetStdHandle (STD_INPUT_HANDLE) } , ENABLE_PROCESSED_INPUT , false ,) }) ; read_key_event () ? } ; let unicode_char = unsafe { key_event . uChar . UnicodeChar } ; if unicode_char == 0 { Ok (key_from_key_code (key_event . wVirtualKeyCode)) } else { match char :: from_utf16_tuple ((unicode_char , None)) { Ok (c) => { if c == '\r' { Ok (Key :: Enter) } else if c == '\t' { Ok (Key :: Tab) } else if c == '\x08' { Ok (Key :: Backspace) } else if c == '\x1B' { Ok (Key :: Escape) } else if c == '\x03' && ctrlc_key { Ok (Key :: CtrlC) } else { Ok (Key :: Char (c)) } } Err (Utf16TupleError :: MissingSecond) => { if get_key_event_count () ? == 0 { let message = format ! ("Read invalid utf16 {}: {}" , unicode_char , Utf16TupleError :: MissingSecond) ; return Err (io :: Error :: new (io :: ErrorKind :: InvalidData , message)) ; } let next_event = read_key_event () ? ; let next_surrogate = unsafe { next_event . uChar . UnicodeChar } ; match char :: from_utf16_tuple ((unicode_char , Some (next_surrogate))) { Ok (c) => Ok (Key :: Char (c)) , Err (e) => { let message = format ! ("Read invalid surrogate pair ({unicode_char}, {next_surrogate}): {e}" ,) ; Err (io :: Error :: new (io :: ErrorKind :: InvalidData , message)) } } } Err (e) => { let message = format ! ("Read invalid utf16 {unicode_char}: {e}") ; Err (io :: Error :: new (io :: ErrorKind :: InvalidData , message)) } } } }
};
}
