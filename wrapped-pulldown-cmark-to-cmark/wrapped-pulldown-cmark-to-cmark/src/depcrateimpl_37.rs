// Generated macro for impl_37 (impl)
macro_rules! Depcrateimpl_37 {
() => {
// Module: crate
// Provides: {"impl_37"}
// Dependencies: {}
impl Options < '_ > { pub fn special_characters (& self) -> Cow < 'static , str > { const BASE : & str = "#\\_*<>`|[]" ; if DEFAULT_OPTIONS . code_block_token == self . code_block_token && DEFAULT_OPTIONS . list_token == self . list_token && DEFAULT_OPTIONS . emphasis_token == self . emphasis_token && DEFAULT_OPTIONS . strong_token == self . strong_token { BASE . into () } else { let mut s = String :: from (BASE) ; s . push (self . code_block_token) ; s . push (self . list_token) ; s . push (self . emphasis_token) ; s . push_str (self . strong_token) ; s . into () } } }
};
}
