macro_rules! deps {
    () => {
        ChangeEstimates!();
        Estimates!();
        ChangeDistributions!();
        Distribution!();
    };
}

macro_rules! ComparisonData {
    () => {
        deps!();
        pub (crate) struct ComparisonData { pub p_value : f64 , pub t_distribution : Distribution < f64 > , pub t_value : f64 , pub relative_estimates : ChangeEstimates , pub relative_distributions : ChangeDistributions , pub significance_threshold : f64 , pub noise_threshold : f64 , pub base_iter_counts : Vec < f64 > , pub base_sample_times : Vec < f64 > , pub base_avg_times : Vec < f64 > , pub base_estimates : Estimates , }
    };
}

ComparisonData!()