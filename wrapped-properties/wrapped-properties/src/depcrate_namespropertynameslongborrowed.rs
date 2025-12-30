// Generated macro for PropertyNamesLongBorrowed (struct)
macro_rules! Depcrate_namesPropertyNamesLongBorrowed {
() => {
// Module: crate::names
// Provides: {"PropertyNamesLongBorrowed"}
// Dependencies: {}
# [doc = " A borrowed wrapper around property value name-to-enum data, returned by"] # [doc = " [`PropertyNamesLong::as_borrowed()`]. More efficient to query."] # [derive (Debug)] pub struct PropertyNamesLongBorrowed < 'a , T : NamedEnumeratedProperty > { map : & 'a T :: DataStructLongBorrowed < 'a > , }
};
}
