macro_rules! deps {
    () => {
        Indexer!();
    };
}

macro_rules! impl_407 {
    () => {
        deps!();
        impl Drop for Indexer < '_ > { fn drop (& mut self) { unsafe { raw :: git_indexer_free (self . raw) ; drop (Box :: from_raw (self . progress_payload_ptr)) } } }
    };
}

impl_407!();