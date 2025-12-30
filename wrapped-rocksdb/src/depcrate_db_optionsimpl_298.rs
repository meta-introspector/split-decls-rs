// Generated macro for impl_298 (impl)
macro_rules! Depcrate_db_optionsimpl_298 {
() => {
// Module: crate::db_options
// Provides: {"impl_298"}
// Dependencies: {}
impl DBPath { # [doc = " Create a new path"] pub fn new < P : AsRef < Path > > (path : P , target_size : u64) -> Result < Self , Error > { let p = to_cpath (path . as_ref ()) . unwrap () ; let dbpath = unsafe { ffi :: rocksdb_dbpath_create (p . as_ptr () , target_size) } ; if dbpath . is_null () { Err (Error :: new (format ! ("Could not create path for storing sst files at location: {}" , path . as_ref () . display ()))) } else { Ok (DBPath { inner : dbpath }) } } }
};
}
