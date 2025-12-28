macro_rules! deps {
    () => {
        Blob!();
        Result!();
        Error!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl io :: Write for Blob < '_ > { # [doc = " Write data into a BLOB incrementally. Will return `Ok(0)` if the end of"] # [doc = " the blob has been reached; consider using `Write::write_all(buf)`"] # [doc = " if you want to get an error if the entirety of the buffer cannot be"] # [doc = " written."] # [doc = ""] # [doc = " This function may only modify the contents of the BLOB; it is not"] # [doc = " possible to increase the size of a BLOB using this API."] # [doc = ""] # [doc = " # Failure"] # [doc = ""] # [doc = " Will return `Err` if the underlying SQLite write call fails."] # [inline] fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { let max_allowed_len = (self . size () - self . pos) as usize ; let n = min (buf . len () , max_allowed_len) as i32 ; if n <= 0 { return Ok (0) ; } let rc = unsafe { ffi :: sqlite3_blob_write (self . blob , buf . as_ptr () as * mut _ , n , self . pos) } ; self . conn . decode_result (rc) . map (| _ | { self . pos += n ; n as usize }) . map_err (io :: Error :: other) } # [inline] fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
    };
}

impl_49!()