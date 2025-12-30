// Generated macro for flush_lit_buf (function)
macro_rules! Depcrate_stringflush_lit_buf {
() => {
// Module: crate::string
// Provides: {"flush_lit_buf"}
// Dependencies: {}
fn flush_lit_buf < I > (it : & mut ConcatIter < '_ , I > ,) -> Option < ParseResult < Vec < u8 > > > { Some (Ok (RegexGeneratorStrategy (Just (mem :: replace (& mut it . buf , vec ! [])) . sboxed () ,))) }
};
}
