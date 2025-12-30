// Generated macro for impl_179 (impl)
macro_rules! Depcrate_tokenimpl_179 {
() => {
// Module: crate::token
// Provides: {"impl_179"}
// Dependencies: {}
impl Printer { pub fn single_token (& mut self , token : Token , group_contents : fn (& mut Self , TokenStream)) { match token { Token :: Group (delimiter , stream) => self . token_group (delimiter , stream , group_contents) , Token :: Ident (ident) => self . ident (& ident) , Token :: Punct (ch , _spacing) => self . token_punct (ch) , Token :: Literal (literal) => self . token_literal (& literal) , } } fn token_group (& mut self , delimiter : Delimiter , stream : TokenStream , group_contents : fn (& mut Self , TokenStream) ,) { self . delimiter_open (delimiter) ; if ! stream . is_empty () { if delimiter == Delimiter :: Brace { self . space () ; } group_contents (self , stream) ; if delimiter == Delimiter :: Brace { self . space () ; } } self . delimiter_close (delimiter) ; } pub fn ident (& mut self , ident : & Ident) { self . word (ident . to_string ()) ; } pub fn token_punct (& mut self , ch : char) { self . word (ch . to_string ()) ; } pub fn token_literal (& mut self , literal : & Literal) { self . word (literal . to_string ()) ; } pub fn delimiter_open (& mut self , delimiter : Delimiter) { self . word (match delimiter { Delimiter :: Parenthesis => "(" , Delimiter :: Brace => "{" , Delimiter :: Bracket => "[" , Delimiter :: None => return , }) ; } pub fn delimiter_close (& mut self , delimiter : Delimiter) { self . word (match delimiter { Delimiter :: Parenthesis => ")" , Delimiter :: Brace => "}" , Delimiter :: Bracket => "]" , Delimiter :: None => return , }) ; } }
};
}
