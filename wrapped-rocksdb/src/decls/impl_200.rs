macro_rules! deps {
    () => {
        OptionsMustOutliveDB!();
        Options!();
    };
}

macro_rules! impl_200 {
    () => {
        deps!();
        impl Default for Options { fn default () -> Self { unsafe { let opts = ffi :: rocksdb_options_create () ; assert ! (! opts . is_null () , "Could not create RocksDB options") ; Self { inner : opts , outlive : OptionsMustOutliveDB :: default () , } } } }
    };
}

impl_200!()