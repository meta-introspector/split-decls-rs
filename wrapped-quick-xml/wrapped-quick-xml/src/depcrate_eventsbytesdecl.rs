// Generated macro for BytesDecl (struct)
macro_rules! Depcrate_eventsBytesDecl {
() => {
// Module: crate::events
// Provides: {"BytesDecl"}
// Dependencies: {}
# [doc = " An XML declaration (`Event::Decl`)."] # [doc = ""] # [doc = " [W3C XML 1.1 Prolog and Document Type Declaration](http://w3.org/TR/xml11/#sec-prolog-dtd)"] # [doc = ""] # [doc = " This event implements `Deref<Target = [u8]>`. The `deref()` implementation"] # [doc = " returns the content of this event between `<?` and `?>`."] # [doc = ""] # [doc = " Note, that inner text will not contain `?>` sequence inside:"] # [doc = ""] # [doc = " ```"] # [doc = " # use quick_xml::events::{BytesDecl, BytesStart, Event};"] # [doc = " # use quick_xml::reader::Reader;"] # [doc = " # use pretty_assertions::assert_eq;"] # [doc = " let mut reader = Reader::from_str(\"<?xml version = '1.0' ?>\");"] # [doc = " let content = \"xml version = '1.0' \";"] # [doc = " let event = BytesDecl::from_start(BytesStart::from_content(content, 3));"] # [doc = ""] # [doc = " assert_eq!(reader.read_event().unwrap(), Event::Decl(event.borrow()));"] # [doc = " // deref coercion of &BytesDecl to &[u8]"] # [doc = " assert_eq!(&event as &[u8], content.as_bytes());"] # [doc = " // AsRef<[u8]> for &T + deref coercion"] # [doc = " assert_eq!(event.as_ref(), content.as_bytes());"] # [doc = " ```"] # [derive (Clone , Debug , Eq , PartialEq)] pub struct BytesDecl < 'a > { content : BytesStart < 'a > , }
};
}
