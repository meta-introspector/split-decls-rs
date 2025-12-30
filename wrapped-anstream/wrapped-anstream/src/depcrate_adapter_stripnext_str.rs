// Generated macro for next_str (function)
macro_rules! Depcrate_adapter_stripnext_str {
() => {
// Module: crate::adapter::strip
// Provides: {"next_str"}
// Dependencies: {}
# [inline] fn next_str < 's > (bytes : & mut & 's [u8] , state : & mut State) -> Option < & 's str > { let offset = bytes . iter () . copied () . position (| b | { let (next_state , action) = state_change (* state , b) ; if next_state != State :: Anywhere { * state = next_state ; } is_printable_bytes (action , b) }) ; let (_ , next) = bytes . split_at (offset . unwrap_or (bytes . len ())) ; * bytes = next ; * state = State :: Ground ; let offset = bytes . iter () . copied () . position (| b | { let (_next_state , action) = state_change (State :: Ground , b) ; ! (is_printable_bytes (action , b) || is_utf8_continuation (b)) }) ; let (printable , next) = bytes . split_at (offset . unwrap_or (bytes . len ())) ; * bytes = next ; if printable . is_empty () { None } else { let printable = unsafe { from_utf8_unchecked (printable , "`bytes` was validated as UTF-8, the parser preserves UTF-8 continuations" ,) } ; Some (printable) } }
};
}
