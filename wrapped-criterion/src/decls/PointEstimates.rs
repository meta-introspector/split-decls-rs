macro_rules! PointEstimates {
    () => {
        pub struct PointEstimates { pub mean : f64 , pub median : f64 , pub median_abs_dev : f64 , pub std_dev : f64 , }
    };
}

PointEstimates!()