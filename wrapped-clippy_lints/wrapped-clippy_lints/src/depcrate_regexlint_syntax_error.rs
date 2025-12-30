// Generated macro for lint_syntax_error (function)
macro_rules! Depcrate_regexlint_syntax_error {
() => {
// Module: crate::regex
// Provides: {"lint_syntax_error"}
// Dependencies: {}
fn lint_syntax_error (cx : & LateContext < '_ > , error : & regex_syntax :: Error , unescaped : & str , base : Span , offset : u8) { let parts : Option < (_ , _ , & dyn Display) > = match & error { regex_syntax :: Error :: Parse (e) => Some ((e . span () , e . auxiliary_span () , e . kind ())) , regex_syntax :: Error :: Translate (e) => Some ((e . span () , None , e . kind ())) , _ => None , } ; let convert_span = | regex_span : & regex_syntax :: ast :: Span | { let offset = u32 :: from (offset) ; let start = base . lo () + BytePos (u32 :: try_from (regex_span . start . offset) . expect ("offset too large") + offset) ; let end = base . lo () + BytePos (u32 :: try_from (regex_span . end . offset) . expect ("offset too large") + offset) ; Span :: new (start , end , base . ctxt () , base . parent ()) } ; if let Some ((primary , auxiliary , kind)) = parts && let Some (literal_snippet) = base . get_source_text (cx) && let Some (inner) = literal_snippet . get (offset as usize ..) && inner . get (.. unescaped . len ()) == Some (unescaped) { let spans = if let Some (auxiliary) = auxiliary { vec ! [convert_span (primary) , convert_span (auxiliary)] } else { vec ! [convert_span (primary)] } ; span_lint (cx , INVALID_REGEX , spans , format ! ("regex syntax error: {kind}")) ; } else { span_lint_and_help (cx , INVALID_REGEX , base , error . to_string () , None , "consider using a raw string literal: `r\"..\"`" ,) ; } }
};
}
