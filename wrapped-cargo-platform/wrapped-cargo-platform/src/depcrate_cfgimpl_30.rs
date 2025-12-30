// Generated macro for impl_30 (impl)
macro_rules! Depcrate_cfgimpl_30 {
() => {
// Module: crate::cfg
// Provides: {"impl_30"}
// Dependencies: {}
impl < 'a > Iterator for Tokenizer < 'a > { type Item = Result < Token < 'a > , ParseError > ; fn next (& mut self) -> Option < Result < Token < 'a > , ParseError > > { loop { match self . s . next () { Some ((_ , ' ')) => { } Some ((_ , '(')) => return Some (Ok (Token :: LeftParen)) , Some ((_ , ')')) => return Some (Ok (Token :: RightParen)) , Some ((_ , ',')) => return Some (Ok (Token :: Comma)) , Some ((_ , '=')) => return Some (Ok (Token :: Equals)) , Some ((start , '"')) => { while let Some ((end , ch)) = self . s . next () { if ch == '"' { return Some (Ok (Token :: String (& self . orig [start + 1 .. end]))) ; } } return Some (Err (ParseError :: new (self . orig , UnterminatedString))) ; } Some ((start , ch)) if is_ident_start (ch) => { let (start , raw) = if ch == 'r' { if let Some (& (_pos , '#')) = self . s . peek () { self . s . next () ; if let Some ((start , ch)) = self . s . next () { if is_ident_start (ch) { (start , true) } else { return Some (Err (ParseError :: new (self . orig , UnexpectedChar (ch) ,))) ; } } else { return Some (Err (ParseError :: new (self . orig , IncompleteExpr ("identifier") ,))) ; } } else { (start , false) } } else { (start , false) } ; while let Some (& (end , ch)) = self . s . peek () { if ! is_ident_rest (ch) { return Some (Ok (Token :: Ident (raw , & self . orig [start .. end]))) ; } else { self . s . next () ; } } return Some (Ok (Token :: Ident (raw , & self . orig [start ..]))) ; } Some ((_ , ch)) => { return Some (Err (ParseError :: new (self . orig , UnexpectedChar (ch)))) ; } None => return None , } } } }
};
}
