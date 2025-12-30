// Generated macro for bytes_regex_parsed (function)
macro_rules! Depcrate_stringbytes_regex_parsed {
() => {
// Module: crate::string
// Provides: {"bytes_regex_parsed"}
// Dependencies: {}
# [doc = " Like `bytes_regex()`, but allows providing a pre-parsed expression."] pub fn bytes_regex_parsed (expr : & Hir) -> ParseResult < Vec < u8 > > { match expr . kind () { Empty => Ok (Just (vec ! []) . sboxed ()) , Literal (lit) => Ok (Just (lit . 0 . to_vec ()) . sboxed ()) , Class (class) => Ok (match class { hir :: Class :: Unicode (class) => { unicode_class_strategy (class) . prop_map (to_bytes) . sboxed () } hir :: Class :: Bytes (class) => { let subs = class . iter () . map (| r | r . start () ..= r . end ()) ; Union :: new (subs) . prop_map (| b | vec ! [b]) . sboxed () } }) , Repetition (rep) => { Ok (vec (bytes_regex_parsed (& rep . sub) ? , to_range (rep) ?) . prop_map (| parts | parts . concat ()) . sboxed ()) } Capture (capture) => bytes_regex_parsed (& capture . sub) . map (| v | v . 0) , Concat (subs) => { let subs = ConcatIter { iter : subs . iter () , buf : vec ! [] , next : None , } ; let ext = | (mut lhs , rhs) : (Vec < _ > , _) | { lhs . extend (rhs) ; lhs } ; Ok (subs . fold (Ok (None) , | accum : Result < _ , Error > , rhs | { Ok (match accum ? { None => Some (rhs ? . sboxed ()) , Some (accum) => { Some ((accum , rhs ?) . prop_map (ext) . sboxed ()) } }) }) ? . unwrap_or_else (| | Just (vec ! []) . sboxed ())) } Alternation (subs) => { Ok (Union :: try_new (subs . iter () . map (bytes_regex_parsed)) ? . sboxed ()) } Look (_) => unsupported ("anchors/boundaries not supported for string generation" ,) , } . map (RegexGeneratorStrategy) }
};
}
