macro_rules! deps {
    () => {
        HistogramData!();
    };
}

macro_rules! impl_416 {
    () => {
        deps!();
        impl Default for HistogramData { fn default () -> Self { let histogram_data_inner = unsafe { ffi :: rocksdb_statistics_histogram_data_create () } ; assert ! (! histogram_data_inner . is_null () , "Could not create RocksDB histogram data") ; Self { inner : histogram_data_inner , } } }
    };
}

impl_416!()