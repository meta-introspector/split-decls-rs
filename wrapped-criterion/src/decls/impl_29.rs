macro_rules! deps {
    () => {
        Function!();
        BenchmarkId!();
        IntoBenchmarkId!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < S : Into < String > > IntoBenchmarkId for S { fn into_benchmark_id (self) -> BenchmarkId { let function_name = self . into () ; assert ! (! function_name . is_empty () , "Function name must not be empty.") ; BenchmarkId { function_name : Some (function_name) , parameter : None , } } }
    };
}

impl_29!();