// Generated macro for impl_2199 (impl)
macro_rules! Depcrate_rowimpl_2199 {
() => {
// Module: crate::row
// Provides: {"impl_2199"}
// Dependencies: {}
impl < 'a , R , DB > NamedRow < 'a , DB > for R where R : Row < 'a , DB > , DB : Backend , { fn get < ST , T > (& self , column_name : & str) -> deserialize :: Result < T > where T : FromSql < ST , DB > , { let field = Row :: get (self , column_name) . ok_or_else (| | format ! ("Column `{column_name}` was not present in query")) ? ; T :: from_nullable_sql (field . value ()) } }
};
}
