macro_rules! deps {
    () => {
        HistogramData!();
    };
}

macro_rules! impl_417 {
    () => {
        deps!();
        impl Drop for HistogramData { fn drop (& mut self) { unsafe { ffi :: rocksdb_statistics_histogram_data_destroy (self . inner) ; } } }
    };
}

impl_417!()