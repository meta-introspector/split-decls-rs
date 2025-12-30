// Generated macro for impl_667 (impl)
macro_rules! Depcrate_shared_util_escapeimpl_667 {
() => {
// Module: crate::shared::util::escape
// Provides: {"impl_667"}
// Dependencies: {}
impl < 'a > core :: fmt :: Display for Bytes < 'a > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { let mut bytes = self . 0 ; while let Some (result) = utf8 :: decode (bytes) { let ch = match result { Ok (ch) => ch , Err (errant_bytes) => { write ! (f , r"\x{:02x}" , errant_bytes [0]) ? ; bytes = & bytes [1 ..] ; continue ; } } ; bytes = & bytes [ch . len_utf8 () ..] ; match ch { '\0' => write ! (f , "\\0") ? , '\x01' ..= '\x7f' => { write ! (f , "{}" , (ch as u8) . escape_ascii ()) ? ; } _ => write ! (f , "{}" , ch . escape_debug ()) ? , } } Ok (()) } }
};
}
