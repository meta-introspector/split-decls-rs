macro_rules! deps {
    () => {
        Result!();
        ToSql!();
        ConnectionRef!();
        Context!();
        Connection!();
    };
}

macro_rules! impl_564 {
    () => {
        deps!();
        impl Context { # [doc = " Set current cell value"] # [inline] pub fn set_result < T : ToSql > (& mut self , value : & T) -> Result < () > { let t = value . to_sql () ? ; unsafe { set_result (self . 0 , & [] , & t) } ; Ok (()) } # [doc = " Determine if column access is for UPDATE"] # [inline] # [must_use] pub fn no_change (& self) -> bool { unsafe { ffi :: sqlite3_vtab_nochange (self . 0) != 0 } } # [doc = " Get the db connection handle via [sqlite3_context_db_handle](https://www.sqlite.org/c3ref/context_db_handle.html)"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This function is unsafe because improper use may impact the Connection."] pub unsafe fn get_connection (& self) -> Result < ConnectionRef < '_ > > { let handle = ffi :: sqlite3_context_db_handle (self . 0) ; Ok (ConnectionRef { conn : Connection :: from_handle (handle) ? , phantom : PhantomData , }) } }
    };
}

impl_564!()