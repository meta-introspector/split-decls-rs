// Generated macro for impl_1000 (impl)
macro_rules! Depcrate_upvarimpl_1000 {
() => {
// Module: crate::upvar
// Provides: {"impl_1000"}
// Dependencies: {}
impl MigrationWarningReason { fn migration_message (& self) -> String { let base = "changes to closure capture in Rust 2021 will affect" ; if ! self . auto_traits . is_empty () && self . drop_order { format ! ("{base} drop order and which traits the closure implements") } else if self . drop_order { format ! ("{base} drop order") } else { format ! ("{base} which traits the closure implements") } } }
};
}
