// Generated macro for impl_201 (impl)
macro_rules! Depcrate_deserializeimpl_201 {
() => {
// Module: crate::deserialize
// Provides: {"impl_201"}
// Dependencies: {}
impl < T , ST , DB > FromStaticSqlRow < ST , DB > for T where DB : Backend , T : FromSql < ST , DB > , ST : SingleValue , { fn build_from_row < 'a > (row : & impl Row < 'a , DB >) -> Result < Self > { use crate :: row :: Field ; let field = row . get (0) . ok_or (crate :: result :: UnexpectedEndOfRow) ? ; T :: from_nullable_sql (field . value ()) . map_err (| e | { if e . is :: < crate :: result :: UnexpectedNullError > () { e } else { Box :: new (crate :: result :: DeserializeFieldError :: new (field , e)) } }) } }
};
}
