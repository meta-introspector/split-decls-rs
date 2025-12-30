// Generated macro for parse_version (function)
macro_rules! Depcrateparse_version {
() => {
// Module: crate
// Provides: {"parse_version"}
// Dependencies: {}
# [inline] # [doc (hidden)] # [allow (missing_docs)] pub fn parse_version (bytes : & mut Bytes) -> Result < u8 > { if let Some (eight) = bytes . peek_n :: < [u8 ; 8] > (8) { const H10 : u64 = u64 :: from_ne_bytes (* b"HTTP/1.0") ; const H11 : u64 = u64 :: from_ne_bytes (* b"HTTP/1.1") ; unsafe { bytes . advance (8) ; } return match u64 :: from_ne_bytes (eight) { H10 => Ok (Status :: Complete (0)) , H11 => Ok (Status :: Complete (1)) , _ => Err (Error :: Version) , } ; } expect ! (bytes . next () == b'H' => Err (Error :: Version)) ; expect ! (bytes . next () == b'T' => Err (Error :: Version)) ; expect ! (bytes . next () == b'T' => Err (Error :: Version)) ; expect ! (bytes . next () == b'P' => Err (Error :: Version)) ; expect ! (bytes . next () == b'/' => Err (Error :: Version)) ; expect ! (bytes . next () == b'1' => Err (Error :: Version)) ; expect ! (bytes . next () == b'.' => Err (Error :: Version)) ; Ok (Status :: Partial) }
};
}
