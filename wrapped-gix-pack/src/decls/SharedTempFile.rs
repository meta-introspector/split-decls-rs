macro_rules! SharedTempFile {
    () => {
        pub (crate) type SharedTempFile = Arc < parking_lot :: Mutex < std :: io :: BufWriter < gix_tempfile :: Handle < Writable > > > > ;
    };
}

SharedTempFile!();