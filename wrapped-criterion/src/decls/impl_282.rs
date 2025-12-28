macro_rules! deps {
    () => {
        ReportContext!();
        ValueFormatter!();
        MeasurementData!();
        BenchmarkId!();
        Report!();
        BencherReport!();
    };
}

macro_rules! impl_282 {
    () => {
        deps!();
        impl Report for BencherReport { fn measurement_start (& self , id : & BenchmarkId , _context : & ReportContext , _sample_count : u64 , _estimate_ns : f64 , _iter_count : u64 ,) { print ! ("test {} ... " , id) ; } fn measurement_complete (& self , _id : & BenchmarkId , _ : & ReportContext , meas : & MeasurementData < '_ > , formatter : & dyn ValueFormatter ,) { let mut values = [meas . absolute_estimates . median . point_estimate , meas . absolute_estimates . std_dev . point_estimate ,] ; let unit = formatter . scale_for_machines (& mut values) ; println ! ("bench: {:>11} {}/iter (+/- {})" , format :: integer (values [0]) , unit , format :: integer (values [1])) ; } fn group_separator (& self) { println ! () ; } }
    };
}

impl_282!()