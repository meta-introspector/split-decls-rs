// Generated macro for escape_braces (function)
macro_rules! Depcrate_write_literalescape_braces {
() => {
// Module: crate::write::literal
// Provides: {"escape_braces"}
// Dependencies: {}
# [doc = " Replaces `{` with `{{` and `}` with `}}`. If `preserve_unicode_escapes` is `true` the braces"] # [doc = " in `\\u{xxxx}` are left unmodified"] # [expect (clippy :: match_same_arms)] fn escape_braces (literal : & str , preserve_unicode_escapes : bool) -> String { # [derive (Clone , Copy)] enum State { Normal , Backslash , UnicodeEscape , } let mut escaped = String :: with_capacity (literal . len ()) ; let mut state = State :: Normal ; for ch in literal . chars () { state = match (ch , state) { ('{' | '}' , State :: Normal) => { escaped . push (ch) ; State :: Normal } , ('\\' , State :: Normal) if preserve_unicode_escapes => State :: Backslash , ('u' , State :: Backslash) => State :: UnicodeEscape , (_ , State :: Backslash) => State :: Normal , ('}' , State :: UnicodeEscape) => State :: Normal , _ => state , } ; escaped . push (ch) ; } escaped }
};
}
