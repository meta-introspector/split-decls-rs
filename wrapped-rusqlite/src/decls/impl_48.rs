macro_rules! deps {
    () => {
        Result!();
        Error!();
        Blob!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl io :: Read for Blob < '_ > { # [doc = " Read data from a BLOB incrementally. Will return Ok(0) if the end of"] # [doc = " the blob has been reached."] # [doc = ""] # [doc = " # Failure"] # [doc = ""] # [doc = " Will return `Err` if the underlying SQLite read call fails."] # [inline] fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { let max_allowed_len = (self . size () - self . pos) as usize ; let n = min (buf . len () , max_allowed_len) as i32 ; if n <= 0 { return Ok (0) ; } let rc = unsafe { ffi :: sqlite3_blob_read (self . blob , buf . as_mut_ptr () . cast () , n , self . pos) } ; self . conn . decode_result (rc) . map (| _ | { self . pos += n ; n as usize }) . map_err (io :: Error :: other) } }
    };
}

impl_48!()