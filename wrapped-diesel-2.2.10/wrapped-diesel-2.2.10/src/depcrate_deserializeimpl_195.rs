// Generated macro for impl_195 (impl)
macro_rules! Depcrate_deserializeimpl_195 {
() => {
// Module: crate::deserialize
// Provides: {"impl_195"}
// Dependencies: {}
impl < DB , T > FromSqlRow < Untyped , DB > for T where DB : Backend , T : QueryableByName < DB > , { fn build_from_row < 'a > (row : & impl Row < 'a , DB >) -> Result < Self > { T :: build (row) } }
};
}
