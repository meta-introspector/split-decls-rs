// Generated macro for impl_167 (impl)
macro_rules! Depcrate_stringimpl_167 {
() => {
// Module: crate::string
// Provides: {"impl_167"}
// Dependencies: {}
# [allow (deprecated)] impl StrSlice for str { fn get_slice < R > (& self , r : R) -> Option < & str > where R : IndexRange , { let start = r . start () . unwrap_or (0) ; let end = r . end () . unwrap_or (self . len ()) ; if start <= end && self . is_char_boundary (start) && self . is_char_boundary (end) { Some (& self [start .. end]) } else { None } } }
};
}
