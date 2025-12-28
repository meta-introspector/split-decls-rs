macro_rules! deps {
    () => {
        EnvOptions!();
    };
}

macro_rules! impl_403 {
    () => {
        deps!();
        impl Default for EnvOptions { fn default () -> Self { let opts = unsafe { ffi :: rocksdb_envoptions_create () } ; Self { inner : opts } } }
    };
}

impl_403!()