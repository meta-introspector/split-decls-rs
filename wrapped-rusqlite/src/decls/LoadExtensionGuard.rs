macro_rules! deps {
    () => {
        Result!();
        Connection!();
    };
}

macro_rules! LoadExtensionGuard {
    () => {
        deps!();
        # [doc = " RAII guard temporarily enabling SQLite extensions to be loaded."] # [doc = ""] # [doc = " ## Example"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " # use rusqlite::{Connection, Result, LoadExtensionGuard};"] # [doc = " # use std::path::{Path};"] # [doc = " fn load_my_extension(conn: &Connection) -> Result<()> {"] # [doc = "     unsafe {"] # [doc = "         let _guard = LoadExtensionGuard::new(conn)?;"] # [doc = "         conn.load_extension(\"trusted/sqlite/extension\", None::<&str>)"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] pub struct LoadExtensionGuard < 'conn > { conn : & 'conn Connection , }
    };
}

LoadExtensionGuard!()