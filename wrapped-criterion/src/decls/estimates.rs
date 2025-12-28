macro_rules! deps {
    () => {
        PointEstimates!();
        Sample!();
        Estimates!();
        Distributions!();
        BenchmarkConfig!();
    };
}

macro_rules! estimates {
    () => {
        deps!();
        fn estimates (avg_times : & Sample < f64 > , config : & BenchmarkConfig) -> (Distributions , Estimates) { fn stats (sample : & Sample < f64 >) -> (f64 , f64 , f64 , f64) { let mean = sample . mean () ; let std_dev = sample . std_dev (Some (mean)) ; let median = sample . percentiles () . median () ; let mad = sample . median_abs_dev (Some (median)) ; (mean , std_dev , median , mad) } let cl = config . confidence_level ; let nresamples = config . nresamples ; let (mean , std_dev , median , mad) = stats (avg_times) ; let points = PointEstimates { mean , median , std_dev , median_abs_dev : mad , } ; let (dist_mean , dist_stddev , dist_median , dist_mad) = elapsed ! ("Bootstrapping the absolute statistics." , avg_times . bootstrap (nresamples , stats)) ; let distributions = Distributions { mean : dist_mean , slope : None , median : dist_median , median_abs_dev : dist_mad , std_dev : dist_stddev , } ; let estimates = build_estimates (& distributions , & points , cl) ; (distributions , estimates) }
    };
}

estimates!();