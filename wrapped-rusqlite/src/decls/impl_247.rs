macro_rules! deps {
    () => {
        Session!();
    };
}

macro_rules! impl_247 {
    () => {
        deps!();
        impl Drop for Session < '_ > { # [inline] fn drop (& mut self) { if self . filter . is_some () { self . table_filter (None :: < fn (& str) -> bool >) ; } unsafe { ffi :: sqlite3session_delete (self . s) } ; } }
    };
}

impl_247!();