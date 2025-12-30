// Generated macro for impl_837 (impl)
macro_rules! Depcrate_vtabimpl_837 {
() => {
// Module: crate::vtab
// Provides: {"impl_837"}
// Dependencies: {}
impl Values < '_ > { # [doc = " Returns the number of values."] # [inline] # [must_use] pub fn len (& self) -> usize { self . args . len () } # [doc = " Returns `true` if there is no value."] # [inline] # [must_use] pub fn is_empty (& self) -> bool { self . args . is_empty () } # [doc = " Returns value at `idx`"] pub fn get < T : FromSql > (& self , idx : usize) -> Result < T > { let arg = self . args [idx] ; let value = unsafe { ValueRef :: from_value (arg) } ; FromSql :: column_result (value) . map_err (| err | match err { FromSqlError :: InvalidType => Error :: InvalidFilterParameterType (idx , value . data_type ()) , FromSqlError :: Other (err) => { Error :: FromSqlConversionFailure (idx , value . data_type () , err) } FromSqlError :: InvalidBlobSize { .. } => { Error :: FromSqlConversionFailure (idx , value . data_type () , Box :: new (err)) } FromSqlError :: OutOfRange (i) => Error :: IntegralValueOutOfRange (idx , i) , }) } # [cfg (feature = "array")] fn get_array (& self , idx : usize) -> Option < array :: Array > { use crate :: types :: Value ; let arg = self . args [idx] ; let ptr = unsafe { ffi :: sqlite3_value_pointer (arg , array :: ARRAY_TYPE) } ; if ptr . is_null () { None } else { Some (unsafe { let ptr = ptr as * const Vec < Value > ; array :: Array :: increment_strong_count (ptr) ; array :: Array :: from_raw (ptr) }) } } # [doc = " Turns `Values` into an iterator."] # [inline] # [must_use] pub fn iter (& self) -> ValueIter < '_ > { ValueIter { iter : self . args . iter () , } } }
};
}
