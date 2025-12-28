macro_rules! io_err {
    () => {
        # [doc = " Classifiers for IO-errors."] pub mod io_err { use std :: io :: ErrorKind ; # [doc = " Return `true` if `err` indicates that the entry doesn't exist on disk. `raw` is used as well"] # [doc = " for additional checks while the variants are outside the MSRV."] pub fn is_not_found (err : ErrorKind , raw_err : Option < i32 >) -> bool { err == ErrorKind :: NotFound || raw_err == Some (20) } }
    };
}

io_err!()