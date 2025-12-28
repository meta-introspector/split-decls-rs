macro_rules! deps {
    () => {
        Comments!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < 'a > Comments < 'a > { pub fn new (sm : & 'a SourceMap , filename : FileName , input : String) -> Comments < 'a > { let mut comments = gather_comments (sm , filename , input) ; comments . reverse () ; Comments { sm , reversed_comments : comments } } fn peek (& self) -> Option < & Comment > { self . reversed_comments . last () } fn next (& mut self) -> Option < Comment > { self . reversed_comments . pop () } fn trailing_comment (& mut self , span : rustc_span :: Span , next_pos : Option < BytePos > ,) -> Option < Comment > { if let Some (cmnt) = self . peek () { if cmnt . style != CommentStyle :: Trailing { return None ; } let span_line = self . sm . lookup_char_pos (span . hi ()) ; let comment_line = self . sm . lookup_char_pos (cmnt . pos) ; let next = next_pos . unwrap_or_else (| | cmnt . pos + BytePos (1)) ; if span . hi () < cmnt . pos && cmnt . pos < next && span_line . line == comment_line . line { return Some (self . next () . unwrap ()) ; } } None } }
    };
}

impl_39!();