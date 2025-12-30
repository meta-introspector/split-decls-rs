// Generated macro for unicode_class_strategy (function)
macro_rules! Depcrate_stringunicode_class_strategy {
() => {
// Module: crate::string
// Provides: {"unicode_class_strategy"}
// Dependencies: {}
fn unicode_class_strategy (class : & hir :: ClassUnicode ,) -> char :: CharStrategy < 'static > { static NONL_RANGES : & [RangeInclusive < char >] = & ['\x00' ..= '\x09' , '\x0B' ..= :: core :: char :: MAX , '\x0B' ..= :: core :: char :: MAX , '\x0B' ..= :: core :: char :: MAX , '\x0B' ..= :: core :: char :: MAX , '\x0B' ..= :: core :: char :: MAX ,] ; let dotnnl = | x : & hir :: ClassUnicodeRange , y : & hir :: ClassUnicodeRange | { x . start () == '\0' && x . end () == '\x09' && y . start () == '\x0B' && y . end () == '\u{10FFFF}' } ; char :: ranges (match class . ranges () { [x , y] if dotnnl (x , y) || dotnnl (y , x) => Cow :: Borrowed (NONL_RANGES) , _ => Cow :: Owned (class . iter () . map (| r | r . start () ..= r . end ()) . collect ()) , }) }
};
}
