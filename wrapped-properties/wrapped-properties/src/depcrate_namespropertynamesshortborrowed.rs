// Generated macro for PropertyNamesShortBorrowed (struct)
macro_rules! Depcrate_namesPropertyNamesShortBorrowed {
() => {
// Module: crate::names
// Provides: {"PropertyNamesShortBorrowed"}
// Dependencies: {}
# [doc = " A borrowed wrapper around property value name-to-enum data, returned by"] # [doc = " [`PropertyNamesShort::as_borrowed()`]. More efficient to query."] # [derive (Debug)] pub struct PropertyNamesShortBorrowed < 'a , T : NamedEnumeratedProperty > { map : & 'a T :: DataStructShortBorrowed < 'a > , }
};
}
