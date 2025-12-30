// Generated macro for CodePointMapDataBorrowed (struct)
macro_rules! Depcrate_code_point_mapCodePointMapDataBorrowed {
() => {
// Module: crate::code_point_map
// Provides: {"CodePointMapDataBorrowed"}
// Dependencies: {}
# [doc = " A borrowed wrapper around code point set data, returned by"] # [doc = " [`CodePointSetData::as_borrowed()`]. More efficient to query."] # [derive (Clone , Copy , Debug)] pub struct CodePointMapDataBorrowed < 'a , T : TrieValue > { map : & 'a PropertyCodePointMap < 'a , T > , }
};
}
