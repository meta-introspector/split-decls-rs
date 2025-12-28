macro_rules! deps {
    () => {
        Blob!();
        Result!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl Blob < '_ > { # [doc = " Move a BLOB handle to a new row."] # [doc = ""] # [doc = " # Failure"] # [doc = ""] # [doc = " Will return `Err` if the underlying SQLite BLOB reopen call fails."] # [inline] pub fn reopen (& mut self , row : i64) -> Result < () > { let rc = unsafe { ffi :: sqlite3_blob_reopen (self . blob , row) } ; if rc != ffi :: SQLITE_OK { return self . conn . decode_result (rc) ; } self . pos = 0 ; Ok (()) } # [doc = " Return the size in bytes of the BLOB."] # [inline] # [must_use] pub fn size (& self) -> i32 { unsafe { ffi :: sqlite3_blob_bytes (self . blob) } } # [doc = " Return the current size in bytes of the BLOB."] # [inline] # [must_use] pub fn len (& self) -> usize { self . size () . try_into () . unwrap () } # [doc = " Return true if the BLOB is empty."] # [inline] # [must_use] pub fn is_empty (& self) -> bool { self . size () == 0 } # [doc = " Close a BLOB handle."] # [doc = ""] # [doc = " Calling `close` explicitly is not required (the BLOB will be closed"] # [doc = " when the `Blob` is dropped), but it is available, so you can get any"] # [doc = " errors that occur."] # [doc = ""] # [doc = " # Failure"] # [doc = ""] # [doc = " Will return `Err` if the underlying SQLite close call fails."] # [inline] pub fn close (mut self) -> Result < () > { self . close_ () } # [inline] fn close_ (& mut self) -> Result < () > { let rc = unsafe { ffi :: sqlite3_blob_close (self . blob) } ; self . blob = ptr :: null_mut () ; self . conn . decode_result (rc) } }
    };
}

impl_47!()