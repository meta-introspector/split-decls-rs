// Generated macro for BytesRef (struct)
macro_rules! Depcrate_eventsBytesRef {
() => {
// Module: crate::events
// Provides: {"BytesRef"}
// Dependencies: {}
# [doc = " Character or general entity reference (`Event::GeneralRef`): `&ref;` or `&#<number>;`."] # [doc = ""] # [doc = " This event implements `Deref<Target = [u8]>`. The `deref()` implementation"] # [doc = " returns the content of this event between `&` and `;`:"] # [doc = ""] # [doc = " ```"] # [doc = " # use quick_xml::events::{BytesRef, Event};"] # [doc = " # use quick_xml::reader::Reader;"] # [doc = " # use pretty_assertions::assert_eq;"] # [doc = " let mut reader = Reader::from_str(r#\"&entity;\"#);"] # [doc = " let content = \"entity\";"] # [doc = " let event = BytesRef::new(content);"] # [doc = ""] # [doc = " assert_eq!(reader.read_event().unwrap(), Event::GeneralRef(event.borrow()));"] # [doc = " // deref coercion of &BytesRef to &[u8]"] # [doc = " assert_eq!(&event as &[u8], content.as_bytes());"] # [doc = " // AsRef<[u8]> for &T + deref coercion"] # [doc = " assert_eq!(event.as_ref(), content.as_bytes());"] # [doc = " ```"] # [derive (Clone , Eq , PartialEq)] pub struct BytesRef < 'a > { content : Cow < 'a , [u8] > , # [doc = " Encoding in which the `content` is stored inside the event."] decoder : Decoder , }
};
}
