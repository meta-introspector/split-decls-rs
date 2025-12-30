// Generated macro for md_parser (function)
macro_rules! Depcratemd_parser {
() => {
// Module: crate
// Provides: {"md_parser"}
// Dependencies: {}
# [doc = " Creates a new markdown parser with the given input."] pub (crate) fn md_parser (input : & str , url : Option < Url >) -> EventIter < '_ > { let mut options = Options :: empty () ; options . insert (Options :: ENABLE_TABLES) ; options . insert (Options :: ENABLE_FOOTNOTES) ; options . insert (Options :: ENABLE_STRIKETHROUGH) ; options . insert (Options :: ENABLE_SMART_PUNCTUATION) ; let parser = Parser :: new_ext (input , options) ; let parser = parser . into_offset_iter () ; let parser = parser . map (move | (event , range) | match event { Event :: Start (Tag :: Link { link_type , dest_url , title , id , }) if ! matches ! (link_type , LinkType :: Email) => (Event :: Start (Tag :: Link { link_type , dest_url : join_url (url . as_ref () , dest_url) , title , id , }) , range ,) , Event :: End (TagEnd :: Link) => (Event :: End (TagEnd :: Link) , range) , _ => (event , range) , }) ; Box :: new (parser) }
};
}
