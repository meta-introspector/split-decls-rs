macro_rules! deps {
    () => {
        IngestExternalFileOptions!();
    };
}

macro_rules! impl_193 {
    () => {
        deps!();
        impl Drop for IngestExternalFileOptions { fn drop (& mut self) { unsafe { ffi :: rocksdb_ingestexternalfileoptions_destroy (self . inner) ; } } }
    };
}

impl_193!();