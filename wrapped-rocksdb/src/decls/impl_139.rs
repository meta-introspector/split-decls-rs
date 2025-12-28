macro_rules! deps {
    () => {
        DBWALIterator!();
        Error!();
    };
}

macro_rules! impl_139 {
    () => {
        deps!();
        impl DBWALIterator { # [doc = " Returns `true` if the iterator is valid. An iterator is invalidated when"] # [doc = " it reaches the end of its defined range, or when it encounters an error."] # [doc = ""] # [doc = " To check whether the iterator encountered an error after `valid` has"] # [doc = " returned `false`, use the [`status`](DBWALIterator::status) method."] # [doc = " `status` will never return an error when `valid` is `true`."] pub fn valid (& self) -> bool { unsafe { ffi :: rocksdb_wal_iter_valid (self . inner) != 0 } } # [doc = " Returns an error `Result` if the iterator has encountered an error"] # [doc = " during operation. When an error is encountered, the iterator is"] # [doc = " invalidated and [`valid`](DBWALIterator::valid) will return `false` when"] # [doc = " called."] pub fn status (& self) -> Result < () , Error > { unsafe { ffi_try ! (ffi :: rocksdb_wal_iter_status (self . inner)) ; } Ok (()) } }
    };
}

impl_139!();