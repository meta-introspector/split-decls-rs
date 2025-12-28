macro_rules! deps {
    () => {
        InterruptHandle!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl InterruptHandle { # [doc = " Interrupt the query currently executing on another thread. This will"] # [doc = " cause that query to fail with a `SQLITE3_INTERRUPT` error."] pub fn interrupt (& self) { let db_handle = self . db_lock . lock () . unwrap () ; if ! db_handle . is_null () { unsafe { ffi :: sqlite3_interrupt (* db_handle) } } } }
    };
}

impl_60!()