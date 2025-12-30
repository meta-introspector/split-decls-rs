// Generated macro for PayloadEvent (enum)
macro_rules! Depcrate_dePayloadEvent {
() => {
// Module: crate::de
// Provides: {"PayloadEvent"}
// Dependencies: {}
# [doc = " Simplified event which contains only these variants that used by deserializer,"] # [doc = " but [`Text`] events not yet fully processed."] # [doc = ""] # [doc = " [`Text`] events should be trimmed if they does not surrounded by the other"] # [doc = " [`Text`] or [`CData`] events. This event contains intermediate state of [`Text`]"] # [doc = " event, where they are trimmed from the start, but not from the end. To trim"] # [doc = " end spaces we should lookahead by one deserializer event (i. e. skip all"] # [doc = " comments and processing instructions)."] # [doc = ""] # [doc = " [`Text`]: Event::Text"] # [doc = " [`CData`]: Event::CData"] # [derive (Clone , Debug , PartialEq , Eq)] pub enum PayloadEvent < 'a > { # [doc = " Start tag (with attributes) `<tag attr=\"value\">`."] Start (BytesStart < 'a >) , # [doc = " End tag `</tag>`."] End (BytesEnd < 'a >) , # [doc = " Escaped character data between tags."] Text (BytesText < 'a >) , # [doc = " Unescaped character data stored in `<![CDATA[...]]>`."] CData (BytesCData < 'a >) , # [doc = " Document type definition data (DTD) stored in `<!DOCTYPE ...>`."] DocType (BytesText < 'a >) , # [doc = " Reference `&ref;` in the textual data."] GeneralRef (BytesRef < 'a >) , # [doc = " End of XML document."] Eof , }
};
}
