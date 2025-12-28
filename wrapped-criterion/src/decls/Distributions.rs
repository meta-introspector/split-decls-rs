macro_rules! deps {
    () => {
        Distribution!();
    };
}

macro_rules! Distributions {
    () => {
        deps!();
        pub struct Distributions { pub mean : Distribution < f64 > , pub median : Distribution < f64 > , pub median_abs_dev : Distribution < f64 > , pub slope : Option < Distribution < f64 > > , pub std_dev : Distribution < f64 > , }
    };
}

Distributions!();