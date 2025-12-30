// Generated macro for ends_with_newline (function)
macro_rules! Depcrate_file_writeends_with_newline {
() => {
// Module: crate::file::write
// Provides: {"ends_with_newline"}
// Dependencies: {}
pub (crate) fn ends_with_newline (e : & [crate :: parse :: Event < '_ >] , nl : impl AsRef < [u8] > , default : bool) -> bool { if e . is_empty () { return default ; } e . iter () . rev () . take_while (| e | e . to_bstr_lossy () . iter () . all (u8 :: is_ascii_whitespace)) . find_map (| e | e . to_bstr_lossy () . contains_str (nl . as_ref ()) . then_some (true)) . unwrap_or (false) }
};
}
