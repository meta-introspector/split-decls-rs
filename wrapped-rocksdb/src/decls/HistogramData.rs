macro_rules! HistogramData {
    () => {
        pub struct HistogramData { pub (crate) inner : * mut ffi :: rocksdb_statistics_histogram_data_t , }
    };
}

HistogramData!()