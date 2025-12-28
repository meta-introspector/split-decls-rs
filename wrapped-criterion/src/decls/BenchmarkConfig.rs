macro_rules! deps {
    () => {
        Duration!();
    };
}

macro_rules! BenchmarkConfig {
    () => {
        deps!();
        # [derive (Debug , Serialize)] pub struct BenchmarkConfig { confidence_level : f64 , measurement_time : Duration , noise_threshold : f64 , nresamples : usize , sample_size : usize , significance_level : f64 , warm_up_time : Duration , }
    };
}

BenchmarkConfig!()