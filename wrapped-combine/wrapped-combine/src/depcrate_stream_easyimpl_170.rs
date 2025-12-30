// Generated macro for impl_170 (impl)
macro_rules! Depcrate_stream_easyimpl_170 {
() => {
// Module: crate::stream::easy
// Provides: {"impl_170"}
// Dependencies: {}
impl < Item , Range > crate :: error :: StreamErrorInto < Item , Range > for Error < Item , Range > { fn into_other_error < T , Item2 , Range2 > (self) -> T where T : crate :: error :: StreamError < Item2 , Range2 > , Item2 : From < Item > , Range2 : From < Range > , { match self { Error :: Unexpected (info) => match info { Info :: Token (x) => T :: unexpected_token (x . into ()) , Info :: Range (x) => T :: unexpected_range (x . into ()) , Info :: Static (x) => T :: unexpected_static_message (x) , Info :: Owned (x) => T :: unexpected_format (x) , } , Error :: Expected (info) => match info { Info :: Token (x) => T :: expected_token (x . into ()) , Info :: Range (x) => T :: expected_range (x . into ()) , Info :: Static (x) => T :: expected_static_message (x) , Info :: Owned (x) => T :: expected_format (x) , } , Error :: Message (info) => match info { Info :: Token (x) => T :: expected_token (x . into ()) , Info :: Range (x) => T :: expected_range (x . into ()) , Info :: Static (x) => T :: expected_static_message (x) , Info :: Owned (x) => T :: expected_format (x) , } , Error :: Other (err) => T :: message_format (err) , } } }
};
}
