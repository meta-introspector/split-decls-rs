// Generated macro for BytesPI (struct)
macro_rules! Depcrate_eventsBytesPI {
() => {
// Module: crate::events
// Provides: {"BytesPI"}
// Dependencies: {}
# [doc = " [Processing instructions][PI] (PIs) allow documents to contain instructions for applications."] # [doc = ""] # [doc = " This event implements `Deref<Target = [u8]>`. The `deref()` implementation"] # [doc = " returns the content of this event between `<?` and `?>`."] # [doc = ""] # [doc = " Note, that inner text will not contain `?>` sequence inside:"] # [doc = ""] # [doc = " ```"] # [doc = " # use quick_xml::events::{BytesPI, Event};"] # [doc = " # use quick_xml::reader::Reader;"] # [doc = " # use pretty_assertions::assert_eq;"] # [doc = " let mut reader = Reader::from_str(\"<?processing instruction >:-<~ ?>\");"] # [doc = " let content = \"processing instruction >:-<~ \";"] # [doc = " let event = BytesPI::new(content);"] # [doc = ""] # [doc = " assert_eq!(reader.read_event().unwrap(), Event::PI(event.borrow()));"] # [doc = " // deref coercion of &BytesPI to &[u8]"] # [doc = " assert_eq!(&event as &[u8], content.as_bytes());"] # [doc = " // AsRef<[u8]> for &T + deref coercion"] # [doc = " assert_eq!(event.as_ref(), content.as_bytes());"] # [doc = " ```"] # [doc = ""] # [doc = " [PI]: https://www.w3.org/TR/xml11/#sec-pi"] # [derive (Clone , Eq , PartialEq)] pub struct BytesPI < 'a > { content : BytesStart < 'a > , }
};
}
