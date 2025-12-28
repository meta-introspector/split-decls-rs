macro_rules! deps {
    () => {
        DBIteratorWithThreadMode!();
        DBRawIteratorWithThreadMode!();
        DBAccess!();
        ReadOptions!();
        IteratorMode!();
        Direction!();
    };
}

macro_rules! impl_134 {
    () => {
        deps!();
        impl < 'a , D : DBAccess > DBIteratorWithThreadMode < 'a , D > { pub (crate) fn new (db : & D , readopts : ReadOptions , mode : IteratorMode) -> Self { Self :: from_raw (DBRawIteratorWithThreadMode :: new (db , readopts) , mode) } pub (crate) fn new_cf (db : & 'a D , cf_handle : * mut ffi :: rocksdb_column_family_handle_t , readopts : ReadOptions , mode : IteratorMode ,) -> Self { Self :: from_raw (DBRawIteratorWithThreadMode :: new_cf (db , cf_handle , readopts) , mode ,) } fn from_raw (raw : DBRawIteratorWithThreadMode < 'a , D > , mode : IteratorMode) -> Self { let mut rv = DBIteratorWithThreadMode { raw , direction : Direction :: Forward , done : false , } ; rv . set_mode (mode) ; rv } pub fn set_mode (& mut self , mode : IteratorMode) { self . done = false ; self . direction = match mode { IteratorMode :: Start => { self . raw . seek_to_first () ; Direction :: Forward } IteratorMode :: End => { self . raw . seek_to_last () ; Direction :: Reverse } IteratorMode :: From (key , Direction :: Forward) => { self . raw . seek (key) ; Direction :: Forward } IteratorMode :: From (key , Direction :: Reverse) => { self . raw . seek_for_prev (key) ; Direction :: Reverse } } ; } }
    };
}

impl_134!();