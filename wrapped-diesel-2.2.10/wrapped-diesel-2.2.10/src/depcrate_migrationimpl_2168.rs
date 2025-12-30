// Generated macro for impl_2168 (impl)
macro_rules! Depcrate_migrationimpl_2168 {
() => {
// Module: crate::migration
// Provides: {"impl_2168"}
// Dependencies: {}
impl MigrationVersion < '_ > { # [doc = " Convert the current migration version into"] # [doc = " an owned variant with static life time"] pub fn as_owned (& self) -> MigrationVersion < 'static > { MigrationVersion (Cow :: Owned (self . 0 . as_ref () . to_owned ())) } }
};
}
