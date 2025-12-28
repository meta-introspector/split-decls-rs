macro_rules! deps {
    () => {
        DBAccess!();
        TransactionDB!();
        ReadOptions!();
        Error!();
        AsColumnFamilyRef!();
        DBPinnableSlice!();
        ThreadMode!();
    };
}

macro_rules! impl_457 {
    () => {
        deps!();
        impl < T : ThreadMode > DBAccess for TransactionDB < T > { unsafe fn create_snapshot (& self) -> * const ffi :: rocksdb_snapshot_t { ffi :: rocksdb_transactiondb_create_snapshot (self . inner) } unsafe fn release_snapshot (& self , snapshot : * const ffi :: rocksdb_snapshot_t) { ffi :: rocksdb_transactiondb_release_snapshot (self . inner , snapshot) ; } unsafe fn create_iterator (& self , readopts : & ReadOptions) -> * mut ffi :: rocksdb_iterator_t { ffi :: rocksdb_transactiondb_create_iterator (self . inner , readopts . inner) } unsafe fn create_iterator_cf (& self , cf_handle : * mut ffi :: rocksdb_column_family_handle_t , readopts : & ReadOptions ,) -> * mut ffi :: rocksdb_iterator_t { ffi :: rocksdb_transactiondb_create_iterator_cf (self . inner , readopts . inner , cf_handle) } fn get_opt < K : AsRef < [u8] > > (& self , key : K , readopts : & ReadOptions ,) -> Result < Option < Vec < u8 > > , Error > { self . get_opt (key , readopts) } fn get_cf_opt < K : AsRef < [u8] > > (& self , cf : & impl AsColumnFamilyRef , key : K , readopts : & ReadOptions ,) -> Result < Option < Vec < u8 > > , Error > { self . get_cf_opt (cf , key , readopts) } fn get_pinned_opt < K : AsRef < [u8] > > (& self , key : K , readopts : & ReadOptions ,) -> Result < Option < DBPinnableSlice < '_ > > , Error > { self . get_pinned_opt (key , readopts) } fn get_pinned_cf_opt < K : AsRef < [u8] > > (& self , cf : & impl AsColumnFamilyRef , key : K , readopts : & ReadOptions ,) -> Result < Option < DBPinnableSlice < '_ > > , Error > { self . get_pinned_cf_opt (cf , key , readopts) } fn multi_get_opt < K , I > (& self , keys : I , readopts : & ReadOptions ,) -> Vec < Result < Option < Vec < u8 > > , Error > > where K : AsRef < [u8] > , I : IntoIterator < Item = K > , { self . multi_get_opt (keys , readopts) } fn multi_get_cf_opt < 'b , K , I , W > (& self , keys_cf : I , readopts : & ReadOptions ,) -> Vec < Result < Option < Vec < u8 > > , Error > > where K : AsRef < [u8] > , I : IntoIterator < Item = (& 'b W , K) > , W : AsColumnFamilyRef + 'b , { self . multi_get_cf_opt (keys_cf , readopts) } }
    };
}

impl_457!();