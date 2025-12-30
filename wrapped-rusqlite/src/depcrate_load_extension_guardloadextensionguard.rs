// Generated macro for LoadExtensionGuard (struct)
macro_rules! Depcrate_load_extension_guardLoadExtensionGuard {
() => {
// Module: crate::load_extension_guard
// Provides: {"LoadExtensionGuard"}
// Dependencies: {}
# [doc = " RAII guard temporarily enabling SQLite extensions to be loaded."] # [doc = ""] # [doc = " ## Example"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " # use rusqlite::{Connection, Result, LoadExtensionGuard};"] # [doc = " # use std::path::{Path};"] # [doc = " fn load_my_extension(conn: &Connection) -> Result<()> {"] # [doc = "     unsafe {"] # [doc = "         let _guard = LoadExtensionGuard::new(conn)?;"] # [doc = "         conn.load_extension(\"trusted/sqlite/extension\", None::<&str>)"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] pub struct LoadExtensionGuard < 'conn > { conn : & 'conn Connection , }
};
}
