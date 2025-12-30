// Generated macro for Header (trait)
macro_rules! DepcrateHeader {
() => {
// Module: crate
// Provides: {"Header"}
// Dependencies: {}
# [doc = " A trait for any object that will represent a header field and value."] # [doc = ""] # [doc = " This trait represents the construction and identification of headers,"] # [doc = " and contains trait-object unsafe methods."] pub trait Header { # [doc = " The name of this header."] fn name () -> & 'static HeaderName ; # [doc = " Decode this type from an iterator of [`HeaderValue`]s."] fn decode < 'i , I > (values : & mut I) -> Result < Self , Error > where Self : Sized , I : Iterator < Item = & 'i HeaderValue > ; # [doc = " Encode this type to a [`HeaderValue`], and add it to a container"] # [doc = " which has [`HeaderValue`] type as each element."] # [doc = ""] # [doc = " This function should be infallible. Any errors converting to a"] # [doc = " `HeaderValue` should have been caught when parsing or constructing"] # [doc = " this value."] fn encode < E : Extend < HeaderValue > > (& self , values : & mut E) ; }
};
}
