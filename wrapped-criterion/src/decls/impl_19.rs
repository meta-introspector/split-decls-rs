macro_rules! deps {
    () => {
        BenchmarkConfig!();
        PartialBenchmarkConfig!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl PartialBenchmarkConfig { pub (crate) fn to_complete (& self , defaults : & BenchmarkConfig) -> BenchmarkConfig { BenchmarkConfig { confidence_level : self . confidence_level . unwrap_or (defaults . confidence_level) , measurement_time : self . measurement_time . unwrap_or (defaults . measurement_time) , noise_threshold : self . noise_threshold . unwrap_or (defaults . noise_threshold) , nresamples : self . nresamples . unwrap_or (defaults . nresamples) , sample_size : self . sample_size . unwrap_or (defaults . sample_size) , significance_level : self . significance_level . unwrap_or (defaults . significance_level) , warm_up_time : self . warm_up_time . unwrap_or (defaults . warm_up_time) , sampling_mode : self . sampling_mode . unwrap_or (defaults . sampling_mode) , quick_mode : self . quick_mode . unwrap_or (defaults . quick_mode) , } } }
    };
}

impl_19!()