macro_rules! deps {
    () => {
        Updates!();
        ConflictMode!();
    };
}

macro_rules! impl_582 {
    () => {
        deps!();
        impl Updates < '_ > { # [doc = " Returns `true` if and only"] # [doc = " - if the column corresponding to `idx` is unchanged by the UPDATE operation that the [`UpdateVTab::update`] method call was invoked to implement"] # [doc = " - and if and the prior [`VTabCursor::column`] method call that was invoked to extracted the value for that column returned without setting a result."] # [inline] # [must_use] # [cfg (feature = "modern_sqlite")] pub fn no_change (& self , idx : usize) -> bool { unsafe { ffi :: sqlite3_value_nochange (self . values . args [idx]) != 0 } } # [doc = " Determine the virtual table conflict policy"] # [doc = ""] # [doc = " # Safety"] # [doc = " This function is unsafe because it uses raw pointer"] # [must_use] pub unsafe fn on_conflict (& self , db : * mut ffi :: sqlite3) -> ConflictMode { ConflictMode :: from (unsafe { ffi :: sqlite3_vtab_on_conflict (db) }) } }
    };
}

impl_582!();