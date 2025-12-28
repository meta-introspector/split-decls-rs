macro_rules! deps {
    () => {
        PerfContext!();
    };
}

macro_rules! impl_289 {
    () => {
        deps!();
        impl Default for PerfContext { fn default () -> Self { let ctx = unsafe { ffi :: rocksdb_perfcontext_create () } ; assert ! (! ctx . is_null () , "Could not create Perf Context") ; Self { inner : ctx } } }
    };
}

impl_289!()