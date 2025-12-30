// Generated macro for impl_704 (impl)
macro_rules! Depcrate_readimpl_704 {
() => {
// Module: crate::read
// Provides: {"impl_704"}
// Dependencies: {}
# [cfg (feature = "std")] impl < R > IoRead < R > where R : io :: Read , { fn parse_str_bytes < 's , T , F > (& 's mut self , scratch : & 's mut Vec < u8 > , validate : bool , result : F ,) -> Result < T > where T : 's , F : FnOnce (& 's Self , & 's [u8]) -> Result < T > , { loop { let ch = tri ! (next_or_eof (self)) ; if ! is_escape (ch , true) { scratch . push (ch) ; continue ; } match ch { b'"' => { return result (self , scratch) ; } b'\\' => { tri ! (parse_escape (self , validate , scratch)) ; } _ => { if validate { return error (self , ErrorCode :: ControlCharacterWhileParsingString) ; } scratch . push (ch) ; } } } } }
};
}
