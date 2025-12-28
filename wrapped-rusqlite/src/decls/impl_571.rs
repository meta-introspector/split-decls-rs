macro_rules! deps {
    () => {
        ValueRef!();
        Result!();
        Error!();
        InValues!();
    };
}

macro_rules! impl_571 {
    () => {
        deps!();
        impl < 'a > fallible_iterator :: FallibleIterator for InValues < 'a > { type Error = Error ; type Item = ValueRef < 'a > ; fn next (& mut self) -> Result < Option < Self :: Item > > { let mut val : * mut ffi :: sqlite3_value = ptr :: null_mut () ; let rc = unsafe { if self . first { self . first = false ; ffi :: sqlite3_vtab_in_first (self . list , & mut val) } else { ffi :: sqlite3_vtab_in_next (self . list , & mut val) } } ; match rc { ffi :: SQLITE_OK => Ok (Some (unsafe { ValueRef :: from_value (val) })) , ffi :: SQLITE_DONE => Ok (None) , _ => Err (error_from_sqlite_code (rc , None)) , } } }
    };
}

impl_571!();