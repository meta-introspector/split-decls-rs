// Generated macro for Event (enum)
macro_rules! Depcrate_eventsEvent {
() => {
// Module: crate::events
// Provides: {"Event"}
// Dependencies: {}
# [doc = " Event emitted by [`Reader::read_event_into`]."] # [doc = ""] # [doc = " [`Reader::read_event_into`]: crate::reader::Reader::read_event_into"] # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub enum Event < 'a > { # [doc = " Start tag (with attributes) `<tag attr=\"value\">`."] Start (BytesStart < 'a >) , # [doc = " End tag `</tag>`."] End (BytesEnd < 'a >) , # [doc = " Empty element tag (with attributes) `<tag attr=\"value\" />`."] Empty (BytesStart < 'a >) , # [doc = " Escaped character data between tags."] Text (BytesText < 'a >) , # [doc = " Unescaped character data stored in `<![CDATA[...]]>`."] CData (BytesCData < 'a >) , # [doc = " Comment `<!-- ... -->`."] Comment (BytesText < 'a >) , # [doc = " XML declaration `<?xml ...?>`."] Decl (BytesDecl < 'a >) , # [doc = " Processing instruction `<?...?>`."] PI (BytesPI < 'a >) , # [doc = " Document type definition data (DTD) stored in `<!DOCTYPE ...>`."] DocType (BytesText < 'a >) , # [doc = " General reference `&entity;` in the textual data. Can be either an entity"] # [doc = " reference, or a character reference."] GeneralRef (BytesRef < 'a >) , # [doc = " End of XML document."] Eof , }
};
}
