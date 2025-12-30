// Generated macro for next_bytes (function)
macro_rules! Depcrate_adapter_stripnext_bytes {
() => {
// Module: crate::adapter::strip
// Provides: {"next_bytes"}
// Dependencies: {}
# [inline] fn next_bytes < 's > (bytes : & mut & 's [u8] , state : & mut State , utf8parser : & mut Utf8Parser ,) -> Option < & 's [u8] > { let offset = bytes . iter () . copied () . position (| b | { if * state == State :: Utf8 { true } else { let (next_state , action) = state_change (* state , b) ; if next_state != State :: Anywhere { * state = next_state ; } is_printable_bytes (action , b) } }) ; let (_ , next) = bytes . split_at (offset . unwrap_or (bytes . len ())) ; * bytes = next ; let offset = bytes . iter () . copied () . position (| b | { if * state == State :: Utf8 { if utf8parser . add (b) { * state = State :: Ground ; } false } else { let (next_state , action) = state_change (State :: Ground , b) ; if next_state != State :: Anywhere { * state = next_state ; } if * state == State :: Utf8 { utf8parser . add (b) ; false } else { ! is_printable_bytes (action , b) } } }) ; let (printable , next) = bytes . split_at (offset . unwrap_or (bytes . len ())) ; * bytes = next ; if printable . is_empty () { None } else { Some (printable) } }
};
}
