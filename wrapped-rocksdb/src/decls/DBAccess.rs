macro_rules! deps {
    () => {
        ReadOptions!();
        AsColumnFamilyRef!();
        DBPinnableSlice!();
        Error!();
    };
}

macro_rules! DBAccess {
    () => {
        deps!();
        # [doc = " Minimal set of DB-related methods, intended to be generic over"] # [doc = " `DBWithThreadMode<T>`. Mainly used internally"] pub trait DBAccess { unsafe fn create_snapshot (& self) -> * const ffi :: rocksdb_snapshot_t ; unsafe fn release_snapshot (& self , snapshot : * const ffi :: rocksdb_snapshot_t) ; unsafe fn create_iterator (& self , readopts : & ReadOptions) -> * mut ffi :: rocksdb_iterator_t ; unsafe fn create_iterator_cf (& self , cf_handle : * mut ffi :: rocksdb_column_family_handle_t , readopts : & ReadOptions ,) -> * mut ffi :: rocksdb_iterator_t ; fn get_opt < K : AsRef < [u8] > > (& self , key : K , readopts : & ReadOptions ,) -> Result < Option < Vec < u8 > > , Error > ; fn get_cf_opt < K : AsRef < [u8] > > (& self , cf : & impl AsColumnFamilyRef , key : K , readopts : & ReadOptions ,) -> Result < Option < Vec < u8 > > , Error > ; fn get_pinned_opt < K : AsRef < [u8] > > (& self , key : K , readopts : & ReadOptions ,) -> Result < Option < DBPinnableSlice < '_ > > , Error > ; fn get_pinned_cf_opt < K : AsRef < [u8] > > (& self , cf : & impl AsColumnFamilyRef , key : K , readopts : & ReadOptions ,) -> Result < Option < DBPinnableSlice < '_ > > , Error > ; fn multi_get_opt < K , I > (& self , keys : I , readopts : & ReadOptions ,) -> Vec < Result < Option < Vec < u8 > > , Error > > where K : AsRef < [u8] > , I : IntoIterator < Item = K > ; fn multi_get_cf_opt < 'b , K , I , W > (& self , keys_cf : I , readopts : & ReadOptions ,) -> Vec < Result < Option < Vec < u8 > > , Error > > where K : AsRef < [u8] > , I : IntoIterator < Item = (& 'b W , K) > , W : AsColumnFamilyRef + 'b ; }
    };
}

DBAccess!();