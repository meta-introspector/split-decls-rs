// Generated macro for impl_2180 (impl)
macro_rules! Depcrate_migrationimpl_2180 {
() => {
// Module: crate::migration
// Provides: {"impl_2180"}
// Dependencies: {}
impl < DB : Backend > Migration < DB > for & dyn Migration < DB > { fn run (& self , conn : & mut dyn BoxableConnection < DB >) -> Result < () > { (* * self) . run (conn) } fn revert (& self , conn : & mut dyn BoxableConnection < DB >) -> Result < () > { (* * self) . revert (conn) } fn metadata (& self) -> & dyn MigrationMetadata { (* * self) . metadata () } fn name (& self) -> & dyn MigrationName { (* * self) . name () } }
};
}
