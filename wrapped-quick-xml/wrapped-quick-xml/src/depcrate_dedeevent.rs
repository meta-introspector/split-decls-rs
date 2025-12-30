// Generated macro for DeEvent (enum)
macro_rules! Depcrate_deDeEvent {
() => {
// Module: crate::de
// Provides: {"DeEvent"}
// Dependencies: {}
# [doc = " Simplified event which contains only these variants that used by deserializer"] # [derive (Clone , Debug , PartialEq , Eq)] pub enum DeEvent < 'a > { # [doc = " Start tag (with attributes) `<tag attr=\"value\">`."] Start (BytesStart < 'a >) , # [doc = " End tag `</tag>`."] End (BytesEnd < 'a >) , # [doc = " Decoded and concatenated content of consequent [`Text`] and [`CData`]"] # [doc = " events. _Consequent_ means that events should follow each other or be"] # [doc = " delimited only by (any count of) [`Comment`] or [`PI`] events."] # [doc = ""] # [doc = " [`Text`]: Event::Text"] # [doc = " [`CData`]: Event::CData"] # [doc = " [`Comment`]: Event::Comment"] # [doc = " [`PI`]: Event::PI"] Text (Text < 'a >) , # [doc = " End of XML document."] Eof , }
};
}
