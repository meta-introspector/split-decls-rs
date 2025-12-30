// Generated macro for impl_11367 (impl)
macro_rules! Depcrate_useless_vecimpl_11367 {
() => {
// Module: crate::useless_vec
// Provides: {"impl_11367"}
// Dependencies: {}
impl SuggestedType { fn desc (self) -> & 'static str { match self { Self :: SliceRef (_) => "a slice" , Self :: Array => "an array" , } } fn snippet (self , cx : & LateContext < '_ > , args_span : Option < Span > , len_span : Option < Span >) -> String { assert ! (args_span . is_none_or (| s | ! s . from_expansion ())) ; assert ! (len_span . is_none_or (| s | ! s . from_expansion ())) ; let maybe_args = args_span . map (| sp | sp . get_source_text (cx) . expect ("spans are always crate-local")) ; let maybe_args = maybe_args . as_deref () . unwrap_or_default () ; let maybe_len = len_span . map (| sp | sp . get_source_text (cx) . expect ("spans are always crate-local")) . map (| st | format ! ("; {st}")) . unwrap_or_default () ; match self { Self :: SliceRef (Mutability :: Mut) => format ! ("&mut [{maybe_args}{maybe_len}]") , Self :: SliceRef (Mutability :: Not) => format ! ("&[{maybe_args}{maybe_len}]") , Self :: Array => format ! ("[{maybe_args}{maybe_len}]") , } } }
};
}
