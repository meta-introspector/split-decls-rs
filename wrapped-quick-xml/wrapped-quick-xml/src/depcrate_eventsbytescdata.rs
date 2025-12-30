// Generated macro for BytesCData (struct)
macro_rules! Depcrate_eventsBytesCData {
() => {
// Module: crate::events
// Provides: {"BytesCData"}
// Dependencies: {}
# [doc = " CDATA content contains unescaped data from the reader. If you want to write them as a text,"] # [doc = " [convert](Self::escape) it to [`BytesText`]."] # [doc = ""] # [doc = " This event implements `Deref<Target = [u8]>`. The `deref()` implementation"] # [doc = " returns the content of this event between `<![CDATA[` and `]]>`."] # [doc = ""] # [doc = " Note, that inner text will not contain `]]>` sequence inside:"] # [doc = ""] # [doc = " ```"] # [doc = " # use quick_xml::events::{BytesCData, Event};"] # [doc = " # use quick_xml::reader::Reader;"] # [doc = " # use pretty_assertions::assert_eq;"] # [doc = " let mut reader = Reader::from_str(\"<![CDATA[ CDATA section ]]>\");"] # [doc = " let content = \" CDATA section \";"] # [doc = " let event = BytesCData::new(content);"] # [doc = ""] # [doc = " assert_eq!(reader.read_event().unwrap(), Event::CData(event.borrow()));"] # [doc = " // deref coercion of &BytesCData to &[u8]"] # [doc = " assert_eq!(&event as &[u8], content.as_bytes());"] # [doc = " // AsRef<[u8]> for &T + deref coercion"] # [doc = " assert_eq!(event.as_ref(), content.as_bytes());"] # [doc = " ```"] # [derive (Clone , Eq , PartialEq)] pub struct BytesCData < 'a > { content : Cow < 'a , [u8] > , # [doc = " Encoding in which the `content` is stored inside the event"] decoder : Decoder , }
};
}
