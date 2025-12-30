// Generated macro for assert_fail (macro)
macro_rules! Depcrate_sqlite_connection_rawassert_fail {
() => {
// Module: crate::sqlite::connection::raw
// Provides: {"assert_fail"}
// Dependencies: {}
# [doc = " For use in FFI function, which cannot unwind."] # [doc = " Print the message, ask to open an issue at Github and [`abort`](std::process::abort)."] macro_rules ! assert_fail { ($ fmt : expr $ (,$ args : tt) *) => { eprint ! (concat ! ($ fmt , "If you see this message, please open an issue at https://github.com/diesel-rs/diesel/issues/new.\n" , "Source location: {}:{}\n" ,) , $ ($ args ,) * file ! () , line ! ()) ; std :: process :: abort () } ; }
};
}
