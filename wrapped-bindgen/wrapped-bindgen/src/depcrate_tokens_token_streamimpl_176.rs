// Generated macro for impl_176 (impl)
macro_rules! Depcrate_tokens_token_streamimpl_176 {
() => {
// Module: crate::tokens::token_stream
// Provides: {"impl_176"}
// Dependencies: {}
impl TokenStream { pub fn new () -> Self { Self (String :: new ()) } # [doc = " Appends another stream to the stream"] # [doc = ""] # [doc = " note: a space will be inserted before the other stream"] pub fn combine < T : AsRef < Self > > (& mut self , other : T) { self . push_space () ; self . 0 . push_str (& other . as_ref () . 0) } # [must_use] pub fn join (& self , value : & str) -> Self { Self (format ! ("{}{value}" , self . 0)) } pub fn is_empty (& self) -> bool { self . 0 . is_empty () } # [doc = " View the stream as a string"] pub fn as_str (& self) -> & str { & self . 0 } # [doc = " Convert the stream into a `String`"] pub fn into_string (self) -> String { self . 0 } # [doc = " Parse the token stream as something"] # [doc = ""] # [doc = " Mostly used with `proc_macro2::TokenStream` or `proc_macro::TokenStream`"] pub fn parse < T : core :: str :: FromStr > (self) -> Result < T , T :: Err > { self . into_string () . parse () } pub (crate) fn push_space (& mut self) { match self . last_char () { None | Some (' ') => { } _ => self . 0 . push (' ') , } } pub fn push (& mut self , c : char) { self . 0 . push (c) } pub fn push_str (& mut self , str : & str) { self . 0 . push_str (str) } fn last_char (& self) -> Option < char > { self . 0 . chars () . last () } }
};
}
