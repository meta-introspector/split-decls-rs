// Generated macro for impl_1865 (impl)
macro_rules! Depcrate_empty_line_afterimpl_1865 {
() => {
// Module: crate::empty_line_after
// Provides: {"impl_1865"}
// Dependencies: {}
impl < 'a > Gap < 'a > { fn new (cx : & EarlyContext < '_ > , prev_chunk : & 'a [Stop] , next_chunk : & 'a [Stop]) -> Option < Self > { let prev_stop = prev_chunk . last () ? ; let next_stop = next_chunk . first () ? ; let gap_span = prev_stop . span . between (next_stop . span) ; let gap_snippet = gap_span . get_source_text (cx) ? ; let mut has_comment = false ; let mut empty_lines = Vec :: new () ; for (token , source , inner_span) in tokenize_with_text (& gap_snippet) { match token { TokenKind :: BlockComment { doc_style : None , terminated : true , } | TokenKind :: LineComment { doc_style : None } => has_comment = true , TokenKind :: Whitespace => { let newlines = source . bytes () . positions (| b | b == b'\n') ; empty_lines . extend (newlines . tuple_windows () . map (| (a , b) | InnerSpan :: new (inner_span . start + a + 1 , inner_span . start + b)) . map (| inner_span | gap_span . from_inner (inner_span)) ,) ; } , _ => return None , } } (! empty_lines . is_empty ()) . then_some (Self { empty_lines , has_comment , next_stop , prev_stop , prev_chunk , }) } fn contiguous_empty_lines (& self) -> impl Iterator < Item = Span > + '_ { self . empty_lines . chunk_by (| a , b | a . hi () + BytePos (1) == b . lo ()) . map (| chunk | { let first = chunk . first () . expect ("at least one empty line") ; let last = chunk . last () . expect ("at least one empty line") ; first . with_lo (first . lo () - BytePos (1)) . with_hi (last . hi ()) }) } }
};
}
