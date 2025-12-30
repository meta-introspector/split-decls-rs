// Generated macro for CodePointSetDataBorrowed (struct)
macro_rules! Depcrate_code_point_setCodePointSetDataBorrowed {
() => {
// Module: crate::code_point_set
// Provides: {"CodePointSetDataBorrowed"}
// Dependencies: {}
# [doc = " A borrowed wrapper around code point set data, returned by"] # [doc = " [`CodePointSetData::as_borrowed()`]. More efficient to query."] # [derive (Clone , Copy , Debug)] pub struct CodePointSetDataBorrowed < 'a > { set : & 'a PropertyCodePointSet < 'a > , }
};
}
