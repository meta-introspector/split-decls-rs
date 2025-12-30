// Generated macro for impl_51 (impl)
macro_rules! Depcrate_features_dieselimpl_51 {
() => {
// Module: crate::features::diesel
// Provides: {"impl_51"}
// Dependencies: {}
impl < ST , DB > deserialize :: FromSql < ST , DB > for CompactString where DB : backend :: Backend , * const str : deserialize :: FromSql < ST , DB > , { fn from_sql (bytes : DB :: RawValue < '_ >) -> deserialize :: Result < Self > { let str_ptr = < * const str as deserialize :: FromSql < ST , DB > > :: from_sql (bytes) ? ; if ! str_ptr . is_null () { let string = unsafe { & * str_ptr } ; Ok (string . into ()) } else { Ok (CompactString :: new ("")) } } }
};
}
