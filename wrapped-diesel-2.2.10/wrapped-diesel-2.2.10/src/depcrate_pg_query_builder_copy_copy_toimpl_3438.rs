// Generated macro for impl_3438 (impl)
macro_rules! Depcrate_pg_query_builder_copy_copy_toimpl_3438 {
() => {
// Module: crate::pg::query_builder::copy::copy_to
// Provides: {"impl_3438"}
// Dependencies: {}
# [cfg (feature = "postgres")] impl < 'f > Field < 'f , Pg > for CopyField < 'f > { fn field_name (& self) -> Option < & str > { None } fn value (& self) -> Option < < Pg as crate :: backend :: Backend > :: RawValue < '_ > > { let value = self . field . as_deref () ? ; Some (crate :: pg :: PgValue :: new_internal (value , self)) } }
};
}
