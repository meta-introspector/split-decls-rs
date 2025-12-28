macro_rules! deps {
    () => {
        WriteBatchWithTransaction!();
        AsColumnFamilyRef!();
    };
}

macro_rules! impl_474 {
    () => {
        deps!();
        impl WriteBatchWithTransaction < false > { # [doc = " Remove database entries from start key to end key."] # [doc = ""] # [doc = " Removes the database entries in the range [\"begin_key\", \"end_key\"), i.e.,"] # [doc = " including \"begin_key\" and excluding \"end_key\". It is not an error if no"] # [doc = " keys exist in the range [\"begin_key\", \"end_key\")."] pub fn delete_range < K : AsRef < [u8] > > (& mut self , from : K , to : K) { let (start_key , end_key) = (from . as_ref () , to . as_ref ()) ; unsafe { ffi :: rocksdb_writebatch_delete_range (self . inner , start_key . as_ptr () as * const c_char , start_key . len () as size_t , end_key . as_ptr () as * const c_char , end_key . len () as size_t ,) ; } } # [doc = " Remove database entries in column family from start key to end key."] # [doc = ""] # [doc = " Removes the database entries in the range [\"begin_key\", \"end_key\"), i.e.,"] # [doc = " including \"begin_key\" and excluding \"end_key\". It is not an error if no"] # [doc = " keys exist in the range [\"begin_key\", \"end_key\")."] pub fn delete_range_cf < K : AsRef < [u8] > > (& mut self , cf : & impl AsColumnFamilyRef , from : K , to : K) { let (start_key , end_key) = (from . as_ref () , to . as_ref ()) ; unsafe { ffi :: rocksdb_writebatch_delete_range_cf (self . inner , cf . inner () , start_key . as_ptr () as * const c_char , start_key . len () as size_t , end_key . as_ptr () as * const c_char , end_key . len () as size_t ,) ; } } }
    };
}

impl_474!()