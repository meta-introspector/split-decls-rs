// Generated macro for impl_754 (impl)
macro_rules! Depcrate_types_value_refimpl_754 {
() => {
// Module: crate::types::value_ref
// Provides: {"impl_754"}
// Dependencies: {}
# [cfg (any (feature = "functions" , feature = "session" , feature = "vtab" , feature = "preupdate_hook"))] impl ValueRef < '_ > { pub (crate) unsafe fn from_value (value : * mut crate :: ffi :: sqlite3_value) -> Self { use crate :: ffi ; use std :: slice :: from_raw_parts ; match ffi :: sqlite3_value_type (value) { ffi :: SQLITE_NULL => ValueRef :: Null , ffi :: SQLITE_INTEGER => ValueRef :: Integer (ffi :: sqlite3_value_int64 (value)) , ffi :: SQLITE_FLOAT => ValueRef :: Real (ffi :: sqlite3_value_double (value)) , ffi :: SQLITE_TEXT => { let text = ffi :: sqlite3_value_text (value) ; let len = ffi :: sqlite3_value_bytes (value) ; assert ! (! text . is_null () , "unexpected SQLITE_TEXT value type with NULL data") ; let s = from_raw_parts (text . cast :: < u8 > () , len as usize) ; ValueRef :: Text (s) } ffi :: SQLITE_BLOB => { let (blob , len) = (ffi :: sqlite3_value_blob (value) , ffi :: sqlite3_value_bytes (value) ,) ; assert ! (len >= 0 , "unexpected negative return from sqlite3_value_bytes") ; if len > 0 { assert ! (! blob . is_null () , "unexpected SQLITE_BLOB value type with NULL data") ; ValueRef :: Blob (from_raw_parts (blob . cast :: < u8 > () , len as usize)) } else { ValueRef :: Blob (& []) } } _ => unreachable ! ("sqlite3_value_type returned invalid value") , } } }
};
}
