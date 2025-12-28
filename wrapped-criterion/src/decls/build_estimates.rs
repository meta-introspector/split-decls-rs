macro_rules! deps {
    () => {
        Estimate!();
        ConfidenceInterval!();
        Distributions!();
        Distribution!();
        Estimates!();
        PointEstimates!();
    };
}

macro_rules! build_estimates {
    () => {
        deps!();
        pub fn build_estimates (distributions : & Distributions , points : & PointEstimates , cl : f64 ,) -> Estimates { let to_estimate = | point_estimate , distribution : & Distribution < f64 > | { let (lb , ub) = distribution . confidence_interval (cl) ; Estimate { confidence_interval : ConfidenceInterval { confidence_level : cl , lower_bound : lb , upper_bound : ub , } , point_estimate , standard_error : distribution . std_dev (None) , } } ; Estimates { mean : to_estimate (points . mean , & distributions . mean) , median : to_estimate (points . median , & distributions . median) , median_abs_dev : to_estimate (points . median_abs_dev , & distributions . median_abs_dev) , slope : None , std_dev : to_estimate (points . std_dev , & distributions . std_dev) , } }
    };
}

build_estimates!();