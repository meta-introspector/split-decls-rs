macro_rules! deps {
    () => {
        IngestExternalFileOptions!();
    };
}

macro_rules! impl_212 {
    () => {
        deps!();
        impl Default for IngestExternalFileOptions { fn default () -> Self { unsafe { Self { inner : ffi :: rocksdb_ingestexternalfileoptions_create () , } } } }
    };
}

impl_212!();