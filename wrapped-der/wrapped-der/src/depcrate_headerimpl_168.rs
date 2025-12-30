// Generated macro for impl_168 (impl)
macro_rules! Depcrate_headerimpl_168 {
() => {
// Module: crate::header
// Provides: {"impl_168"}
// Dependencies: {}
impl Header { # [doc = " Create a new [`Header`] from a [`Tag`] and a [`Length`]."] pub fn new (tag : Tag , length : Length) -> Self { # [cfg (feature = "ber")] let constructed = tag . is_constructed () || length . is_indefinite () ; # [cfg (not (feature = "ber"))] let constructed = tag . is_constructed () ; Self { tag , length , constructed , } } # [doc = " [`Tag`] of this header."] pub fn tag (& self) -> Tag { self . tag } # [doc = " [`Length`] of this header."] pub fn length (& self) -> Length { self . length } # [doc = " True if the [`Tag`] of this header has its constructed bit set."] pub fn is_constructed (& self) -> bool { self . constructed } # [doc = " Copy of header with adjusted length."] pub fn with_length (& self , length : Length) -> Self { Self { tag : self . tag , length , constructed : self . constructed , } } # [doc = " Peek forward in the reader, attempting to decode a [`Header`] at the current position."] # [doc = ""] # [doc = " Does not modify the reader's state."] pub fn peek < 'a > (reader : & impl Reader < 'a >) -> Result < Self > { Header :: decode (& mut reader . clone ()) } }
};
}
