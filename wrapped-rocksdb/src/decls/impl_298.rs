macro_rules! deps {
    () => {
        ThreadMode!();
        TransactionDB!();
        DB!();
        DBCommon!();
        DBInner!();
        MemoryUsage!();
        Cache!();
        MemoryUsageBuilder!();
        Error!();
    };
}

macro_rules! impl_298 {
    () => {
        deps!();
        impl MemoryUsageBuilder { # [doc = " Create new instance"] pub fn new () -> Result < Self , Error > { let mc = unsafe { ffi :: rocksdb_memory_consumers_create () } ; if mc . is_null () { Err (Error :: new ("Could not create MemoryUsage builder" . to_owned () ,)) } else { Ok (Self { inner : mc }) } } # [doc = " Add a DB instance to collect memory usage from it and add up in total stats"] pub fn add_tx_db < T : ThreadMode > (& mut self , db : & TransactionDB < T >) { unsafe { let base = ffi :: rocksdb_transactiondb_get_base_db (db . inner) ; ffi :: rocksdb_memory_consumers_add_db (self . inner , base) ; } } # [doc = " Add a DB instance to collect memory usage from it and add up in total stats"] pub fn add_db < T : ThreadMode , D : DBInner > (& mut self , db : & DBCommon < T , D >) { unsafe { ffi :: rocksdb_memory_consumers_add_db (self . inner , db . inner . inner ()) ; } } # [doc = " Add a cache to collect memory usage from it and add up in total stats"] pub fn add_cache (& mut self , cache : & Cache) { unsafe { ffi :: rocksdb_memory_consumers_add_cache (self . inner , cache . 0 . inner . as_ptr ()) ; } } # [doc = " Build up MemoryUsage"] pub fn build (& self) -> Result < MemoryUsage , Error > { unsafe { let mu = ffi_try ! (ffi :: rocksdb_approximate_memory_usage_create (self . inner)) ; Ok (MemoryUsage { inner : mu }) } } }
    };
}

impl_298!();