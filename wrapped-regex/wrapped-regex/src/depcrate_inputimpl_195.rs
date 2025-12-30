// Generated macro for impl_195 (impl)
macro_rules! Depcrate_inputimpl_195 {
() => {
// Module: crate::input
// Provides: {"impl_195"}
// Dependencies: {}
impl < 't > Input for CharInput < 't > { fn at (& self , i : usize) -> InputAt { let c = decode_utf8 (& self [i ..]) . map (| (c , _) | c) . into () ; InputAt { pos : i , c : c , byte : None , len : c . len_utf8 () , } } fn next_char (& self , at : InputAt) -> Char { at . char () } fn previous_char (& self , at : InputAt) -> Char { decode_last_utf8 (& self [.. at . pos ()]) . map (| (c , _) | c) . into () } fn is_empty_match (& self , at : InputAt , empty : & InstEmptyLook) -> bool { use prog :: EmptyLook :: * ; match empty . look { StartLine => { let c = self . previous_char (at) ; at . pos () == 0 || c == '\n' } EndLine => { let c = self . next_char (at) ; at . pos () == self . len () || c == '\n' } StartText => at . pos () == 0 , EndText => at . pos () == self . len () , WordBoundary => { let (c1 , c2) = (self . previous_char (at) , self . next_char (at)) ; c1 . is_word_char () != c2 . is_word_char () } NotWordBoundary => { let (c1 , c2) = (self . previous_char (at) , self . next_char (at)) ; c1 . is_word_char () == c2 . is_word_char () } WordBoundaryAscii => { let (c1 , c2) = (self . previous_char (at) , self . next_char (at)) ; c1 . is_word_byte () != c2 . is_word_byte () } NotWordBoundaryAscii => { let (c1 , c2) = (self . previous_char (at) , self . next_char (at)) ; c1 . is_word_byte () == c2 . is_word_byte () } } } fn prefix_at (& self , prefixes : & LiteralSearcher , at : InputAt ,) -> Option < InputAt > { prefixes . find (& self [at . pos () ..]) . map (| (s , _) | self . at (at . pos () + s)) } fn len (& self) -> usize { self . 0 . len () } fn as_bytes (& self) -> & [u8] { self . 0 } }
};
}
