// Generated macro for impl_202 (impl)
macro_rules! Depcrate_deserializeimpl_202 {
() => {
// Module: crate::deserialize
// Provides: {"impl_202"}
// Dependencies: {}
impl < T , ST , DB > StaticallySizedRow < ST , DB > for T where ST : SqlTypeOrSelectable + crate :: util :: TupleSize , T : Queryable < ST , DB > , DB : Backend , { const FIELD_COUNT : usize = < ST as crate :: util :: TupleSize > :: SIZE ; }
};
}
