macro_rules! deps {
    () => {
        CachedStatement!();
        Result!();
        Connection!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl Connection { # [doc = " Prepare a SQL statement for execution, returning a previously prepared"] # [doc = " (but not currently in-use) statement if one is available. The"] # [doc = " returned statement will be cached for reuse by future calls to"] # [doc = " [`prepare_cached`](Connection::prepare_cached) once it is dropped."] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " # use rusqlite::{Connection, Result};"] # [doc = " fn insert_new_people(conn: &Connection) -> Result<()> {"] # [doc = "     {"] # [doc = "         let mut stmt = conn.prepare_cached(\"INSERT INTO People (name) VALUES (?1)\")?;"] # [doc = "         stmt.execute([\"Joe Smith\"])?;"] # [doc = "     }"] # [doc = "     {"] # [doc = "         // This will return the same underlying SQLite statement handle without"] # [doc = "         // having to prepare it again."] # [doc = "         let mut stmt = conn.prepare_cached(\"INSERT INTO People (name) VALUES (?1)\")?;"] # [doc = "         stmt.execute([\"Bob Jones\"])?;"] # [doc = "     }"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " # Failure"] # [doc = ""] # [doc = " Will return `Err` if `sql` cannot be converted to a C-compatible string"] # [doc = " or if the underlying SQLite call fails."] # [inline] pub fn prepare_cached (& self , sql : & str) -> Result < CachedStatement < '_ > > { self . cache . get (self , sql) } # [doc = " Set the maximum number of cached prepared statements this connection"] # [doc = " will hold. By default, a connection will hold a relatively small"] # [doc = " number of cached statements. If you need more, or know that you"] # [doc = " will not use cached statements, you"] # [doc = " can set the capacity manually using this method."] # [inline] pub fn set_prepared_statement_cache_capacity (& self , capacity : usize) { self . cache . set_capacity (capacity) ; } # [doc = " Remove/finalize all prepared statements currently in the cache."] # [inline] pub fn flush_prepared_statement_cache (& self) { self . cache . flush () ; } }
    };
}

impl_60!();