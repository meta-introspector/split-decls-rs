macro_rules! deps {
    () => {
        Duration!();
        PlotConfiguration!();
        SamplingMode!();
    };
}

macro_rules! PartialBenchmarkConfig {
    () => {
        deps!();
        # [doc = " Struct representing a partially-complete per-benchmark configuration."] # [derive (Clone , Default)] pub (crate) struct PartialBenchmarkConfig { pub (crate) confidence_level : Option < f64 > , pub (crate) measurement_time : Option < Duration > , pub (crate) noise_threshold : Option < f64 > , pub (crate) nresamples : Option < usize > , pub (crate) sample_size : Option < usize > , pub (crate) significance_level : Option < f64 > , pub (crate) warm_up_time : Option < Duration > , pub (crate) sampling_mode : Option < SamplingMode > , pub (crate) quick_mode : Option < bool > , pub (crate) plot_config : PlotConfiguration , }
    };
}

PartialBenchmarkConfig!()