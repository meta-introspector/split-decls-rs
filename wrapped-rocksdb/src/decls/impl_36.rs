macro_rules! deps {
    () => {
        Error!();
        Checkpoint!();
        DBInner!();
        DBCommon!();
        ThreadMode!();
        DB!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl < 'db > Checkpoint < 'db > { # [doc = " Creates new checkpoint object for specific DB."] # [doc = ""] # [doc = " Does not actually produce checkpoints, call `.create_checkpoint()` method to produce"] # [doc = " a DB checkpoint."] pub fn new < T : ThreadMode , I : DBInner > (db : & 'db DBCommon < T , I >) -> Result < Self , Error > { let checkpoint : * mut ffi :: rocksdb_checkpoint_t ; unsafe { checkpoint = ffi_try ! (ffi :: rocksdb_checkpoint_object_create (db . inner . inner ())) ; } if checkpoint . is_null () { return Err (Error :: new ("Could not create checkpoint object." . to_owned ())) ; } Ok (Self { inner : checkpoint , _db : PhantomData , }) } # [doc = " Creates new physical DB checkpoint in directory specified by `path`."] pub fn create_checkpoint < P : AsRef < Path > > (& self , path : P) -> Result < () , Error > { let cpath = to_cpath (path) ? ; unsafe { ffi_try ! (ffi :: rocksdb_checkpoint_create (self . inner , cpath . as_ptr () , LOG_SIZE_FOR_FLUSH ,)) ; } Ok (()) } }
    };
}

impl_36!()