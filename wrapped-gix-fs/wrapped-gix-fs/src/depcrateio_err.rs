// Generated macro for io_err (module)
macro_rules! Depcrateio_err {
() => {
// Module: crate
// Provides: {"io_err"}
// Dependencies: {}
# [doc = " Classifiers for IO-errors."] pub mod io_err { use std :: io :: ErrorKind ; # [doc = " Return `true` if `err` indicates that the entry doesn't exist on disk. `raw` is used as well"] # [doc = " for additional checks while the variants are outside the MSRV."] pub fn is_not_found (err : ErrorKind , raw_err : Option < i32 >) -> bool { err == ErrorKind :: NotFound || raw_err == Some (20) } }
};
}
