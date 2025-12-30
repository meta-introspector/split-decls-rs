// Generated macro for header_text (function)
macro_rules! Depcrate_utilheader_text {
() => {
// Module: crate::util
// Provides: {"header_text"}
// Dependencies: {}
# [doc = " Extracts the text from a header after `Tag::Heading` has been received."] pub fn header_text < 'e > (parser : & mut EventIter < 'e >) -> Result < CowStr < 'e > , Error > { let text = match parser . next () { Some ((Event :: Text (t) , _range)) => t , e => bail ! ("expected plain text in man header, got {:?}" , e) , } ; match parser . next () { Some ((Event :: End (TagEnd :: Heading (..)) , _range)) => { return Ok (text) ; } e => bail ! ("expected plain text in man header, got {:?}" , e) , } }
};
}
