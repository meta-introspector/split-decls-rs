// Generated macro for unescape (function)
macro_rules! Depcrateunescape {
() => {
// Module: crate
// Provides: {"unescape"}
// Dependencies: {}
fn unescape (mut literal : & str , span : Span) -> parse :: Result < Cow < str > > { if literal . contains ('}') { let mut buf = String :: new () ; while literal . contains ('}') { const ERR : & str = "format string contains an unmatched right brace" ; let mut parts = literal . splitn (2 , '}') ; match (parts . next () , parts . next ()) { (Some (left) , Some (right)) => { const ESCAPED_BRACE : & str = "}" ; if let Some (tail) = right . strip_prefix (ESCAPED_BRACE) { buf . push_str (left) ; buf . push ('}') ; literal = tail ; } else { return Err (parse :: Error :: new (span , ERR)) ; } } _ => unreachable ! () , } } buf . push_str (literal) ; Ok (buf . into ()) } else { Ok (Cow :: Borrowed (literal)) } }
};
}
