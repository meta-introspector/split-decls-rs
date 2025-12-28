macro_rules! deps {
    () => {
        Result!();
        VTabConnection!();
        VTabConfig!();
        Connection!();
    };
}

macro_rules! impl_540 {
    () => {
        deps!();
        impl VTabConnection { # [doc = " Configure various facets of the virtual table interface"] pub fn config (& mut self , config : VTabConfig) -> Result < () > { check (unsafe { ffi :: sqlite3_vtab_config (self . 0 , config as c_int) }) } # [doc = " Get access to the underlying SQLite database connection handle."] # [doc = ""] # [doc = " # Warning"] # [doc = ""] # [doc = " You should not need to use this function. If you do need to, please"] # [doc = " [open an issue on the rusqlite repository](https://github.com/rusqlite/rusqlite/issues) and describe"] # [doc = " your use case."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This function is unsafe because it gives you raw access"] # [doc = " to the SQLite connection, and what you do with it could impact the"] # [doc = " safety of this `Connection`."] pub unsafe fn handle (& mut self) -> * mut ffi :: sqlite3 { self . 0 } }
    };
}

impl_540!();