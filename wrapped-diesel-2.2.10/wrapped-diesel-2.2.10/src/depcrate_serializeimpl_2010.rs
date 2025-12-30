// Generated macro for impl_2010 (impl)
macro_rules! Depcrate_serializeimpl_2010 {
() => {
// Module: crate::serialize
// Provides: {"impl_2010"}
// Dependencies: {}
impl < A , T , DB > ToSql < A , DB > for & T where DB : Backend , T : ToSql < A , DB > + ? Sized , { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , DB >) -> Result { (* self) . to_sql (out) } }
};
}
