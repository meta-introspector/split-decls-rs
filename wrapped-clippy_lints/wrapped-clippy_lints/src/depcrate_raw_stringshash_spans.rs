// Generated macro for hash_spans (function)
macro_rules! Depcrate_raw_stringshash_spans {
() => {
// Module: crate::raw_strings
// Provides: {"hash_spans"}
// Dependencies: {}
# [doc = " Returns spans pointing at the unneeded hashes, e.g. for a `req` of `1` and `max` of `3`:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " r###\"..\"###"] # [doc = "   ^^    ^^"] # [doc = " ```"] fn hash_spans (literal_span : Span , prefix_len : usize , req : u8 , max : u8) -> (Span , Span) { let literal_span = literal_span . data () ; let hash_start = literal_span . lo + BytePos :: from_usize (prefix_len) ; let hash_end = literal_span . hi ; let start = Span :: new (hash_start + BytePos (req . into ()) , hash_start + BytePos (max . into ()) , literal_span . ctxt , None ,) ; let end = Span :: new (hash_end - BytePos (req . into ()) , hash_end - BytePos (max . into ()) , literal_span . ctxt , None ,) ; (start , end) }
};
}
