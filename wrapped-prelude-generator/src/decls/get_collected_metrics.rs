macro_rules! deps {
    () => {
        FunctionMetrics!();
    };
}

macro_rules! get_collected_metrics {
    () => {
        deps!();
        pub fn get_collected_metrics () -> HashMap < String , FunctionMetrics > { METRICS . lock () . unwrap () . clone () }
    };
}

get_collected_metrics!()