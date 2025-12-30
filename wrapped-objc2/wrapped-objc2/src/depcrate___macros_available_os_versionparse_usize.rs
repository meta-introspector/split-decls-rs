// Generated macro for parse_usize (function)
macro_rules! Depcrate___macros_available_os_versionparse_usize {
() => {
// Module: crate::__macros::available::os_version
// Provides: {"parse_usize"}
// Dependencies: {}
# [track_caller] const fn parse_usize (mut bytes : & [u8]) -> (usize , & [u8]) { let mut ret : usize = if let Some ((& ascii , rest)) = bytes . split_first () { bytes = rest ; match ascii { b'0' ..= b'9' => (ascii - b'0') as usize , _ => panic ! ("found invalid digit when parsing version") , } } else { panic ! ("found empty version number part") } ; while let Some ((& ascii , rest)) = bytes . split_first () { let digit = match ascii { b'0' ..= b'9' => ascii - b'0' , _ => break , } ; bytes = rest ; match ret . checked_mul (10) { Some (val) => match val . checked_add (digit as _) { Some (val) => ret = val , None => panic ! ("version is too large") , } , None => panic ! ("version is too large") , } ; } (ret , bytes) }
};
}
