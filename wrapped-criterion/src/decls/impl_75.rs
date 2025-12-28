macro_rules! deps {
    () => {
        BenchmarkConfig!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl From < & crate :: benchmark :: BenchmarkConfig > for BenchmarkConfig { fn from (other : & crate :: benchmark :: BenchmarkConfig) -> Self { BenchmarkConfig { confidence_level : other . confidence_level , measurement_time : other . measurement_time . into () , noise_threshold : other . noise_threshold , nresamples : other . nresamples , sample_size : other . sample_size , significance_level : other . significance_level , warm_up_time : other . warm_up_time . into () , } } }
    };
}

impl_75!();