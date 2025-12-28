macro_rules! deps {
    () => {
        DBPath!();
        Error!();
    };
}

macro_rules! impl_242 {
    () => {
        deps!();
        impl DBPath { # [doc = " Create a new path"] pub fn new < P : AsRef < Path > > (path : P , target_size : u64) -> Result < Self , Error > { let p = to_cpath (path . as_ref ()) . unwrap () ; let dbpath = unsafe { ffi :: rocksdb_dbpath_create (p . as_ptr () , target_size) } ; if dbpath . is_null () { Err (Error :: new (format ! ("Could not create path for storing sst files at location: {}" , path . as_ref () . display ()))) } else { Ok (DBPath { inner : dbpath }) } } }
    };
}

impl_242!();