// Generated macro for Text (struct)
macro_rules! Depcrate_deText {
() => {
// Module: crate::de
// Provides: {"Text"}
// Dependencies: {}
# [doc = " Decoded and concatenated content of consequent [`Text`] and [`CData`]"] # [doc = " events. _Consequent_ means that events should follow each other or be"] # [doc = " delimited only by (any count of) [`Comment`] or [`PI`] events."] # [doc = ""] # [doc = " Internally text is stored in `Cow<str>`. Cloning of text is cheap while it"] # [doc = " is borrowed and makes copies of data when it is owned."] # [doc = ""] # [doc = " [`Text`]: Event::Text"] # [doc = " [`CData`]: Event::CData"] # [doc = " [`Comment`]: Event::Comment"] # [doc = " [`PI`]: Event::PI"] # [derive (Clone , Debug , PartialEq , Eq)] pub struct Text < 'a > { # [doc = " Untrimmed text after concatenating content of all"] # [doc = " [`Text`] and [`CData`] events"] # [doc = ""] # [doc = " [`Text`]: Event::Text"] # [doc = " [`CData`]: Event::CData"] text : Cow < 'a , str > , # [doc = " A range into `text` which contains data after trimming"] content : Range < usize > , }
};
}
