macro_rules! deps {
    () => {
        ThreadMode!();
        DBInner!();
        DBCommon!();
        AsColumnFamilyRef!();
        Error!();
        DBAccess!();
        DBPinnableSlice!();
        ReadOptions!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl < T : ThreadMode , D : DBInner > DBAccess for DBCommon < T , D > { unsafe fn create_snapshot (& self) -> * const ffi :: rocksdb_snapshot_t { unsafe { ffi :: rocksdb_create_snapshot (self . inner . inner ()) } } unsafe fn release_snapshot (& self , snapshot : * const ffi :: rocksdb_snapshot_t) { unsafe { ffi :: rocksdb_release_snapshot (self . inner . inner () , snapshot) } ; } unsafe fn create_iterator (& self , readopts : & ReadOptions) -> * mut ffi :: rocksdb_iterator_t { unsafe { ffi :: rocksdb_create_iterator (self . inner . inner () , readopts . inner) } } unsafe fn create_iterator_cf (& self , cf_handle : * mut ffi :: rocksdb_column_family_handle_t , readopts : & ReadOptions ,) -> * mut ffi :: rocksdb_iterator_t { unsafe { ffi :: rocksdb_create_iterator_cf (self . inner . inner () , readopts . inner , cf_handle) } } fn get_opt < K : AsRef < [u8] > > (& self , key : K , readopts : & ReadOptions ,) -> Result < Option < Vec < u8 > > , Error > { self . get_opt (key , readopts) } fn get_cf_opt < K : AsRef < [u8] > > (& self , cf : & impl AsColumnFamilyRef , key : K , readopts : & ReadOptions ,) -> Result < Option < Vec < u8 > > , Error > { self . get_cf_opt (cf , key , readopts) } fn get_pinned_opt < K : AsRef < [u8] > > (& self , key : K , readopts : & ReadOptions ,) -> Result < Option < DBPinnableSlice < '_ > > , Error > { self . get_pinned_opt (key , readopts) } fn get_pinned_cf_opt < K : AsRef < [u8] > > (& self , cf : & impl AsColumnFamilyRef , key : K , readopts : & ReadOptions ,) -> Result < Option < DBPinnableSlice < '_ > > , Error > { self . get_pinned_cf_opt (cf , key , readopts) } fn multi_get_opt < K , Iter > (& self , keys : Iter , readopts : & ReadOptions ,) -> Vec < Result < Option < Vec < u8 > > , Error > > where K : AsRef < [u8] > , Iter : IntoIterator < Item = K > , { self . multi_get_opt (keys , readopts) } fn multi_get_cf_opt < 'b , K , Iter , W > (& self , keys_cf : Iter , readopts : & ReadOptions ,) -> Vec < Result < Option < Vec < u8 > > , Error > > where K : AsRef < [u8] > , Iter : IntoIterator < Item = (& 'b W , K) > , W : AsColumnFamilyRef + 'b , { self . multi_get_cf_opt (keys_cf , readopts) } }
    };
}

impl_102!()