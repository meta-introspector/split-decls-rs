macro_rules! deps {
    () => {
        Inserts!();
        ConflictMode!();
    };
}

macro_rules! impl_579 {
    () => {
        deps!();
        impl Inserts < '_ > { # [doc = " Determine the virtual table conflict policy"] # [doc = ""] # [doc = " # Safety"] # [doc = " This function is unsafe because it uses raw pointer"] # [must_use] pub unsafe fn on_conflict (& self , db : * mut ffi :: sqlite3) -> ConflictMode { ConflictMode :: from (unsafe { ffi :: sqlite3_vtab_on_conflict (db) }) } }
    };
}

impl_579!()