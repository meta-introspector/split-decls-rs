// Generated macro for impl_10984 (impl)
macro_rules! Depcrate_vecimpl_10984 {
() => {
// Module: crate::vec
// Provides: {"impl_10984"}
// Dependencies: {}
impl SuggestedType { fn desc (self) -> & 'static str { match self { Self :: SliceRef (_) => "a slice" , Self :: Array => "an array" , } } fn snippet (self , cx : & LateContext < '_ > , args_span : Option < Span > , len_span : Option < Span >) -> String { assert ! (args_span . is_none_or (| s | ! s . from_expansion ())) ; assert ! (len_span . is_none_or (| s | ! s . from_expansion ())) ; let maybe_args = args_span . map (| sp | sp . get_source_text (cx) . expect ("spans are always crate-local")) . map_or (String :: new () , | x | x . to_owned ()) ; let maybe_len = len_span . map (| sp | sp . get_source_text (cx) . expect ("spans are always crate-local")) . map (| st | format ! ("; {st}")) . unwrap_or_default () ; match self { Self :: SliceRef (Mutability :: Mut) => format ! ("&mut [{maybe_args}{maybe_len}]") , Self :: SliceRef (Mutability :: Not) => format ! ("&[{maybe_args}{maybe_len}]") , Self :: Array => format ! ("[{maybe_args}{maybe_len}]") , } } }
};
}
