macro_rules! deps {
    () => {
        Reports!();
        Report!();
        BenchmarkId!();
        ReportContext!();
        MeasurementData!();
        ValueFormatter!();
    };
}

macro_rules! impl_276 {
    () => {
        deps!();
        impl Report for Reports { reports_impl ! (fn test_start (& self , id : & BenchmarkId , context : & ReportContext)) ; reports_impl ! (fn test_pass (& self , id : & BenchmarkId , context : & ReportContext)) ; reports_impl ! (fn benchmark_start (& self , id : & BenchmarkId , context : & ReportContext)) ; reports_impl ! (fn profile (& self , id : & BenchmarkId , context : & ReportContext , profile_ns : f64)) ; reports_impl ! (fn warmup (& self , id : & BenchmarkId , context : & ReportContext , warmup_ns : f64)) ; reports_impl ! (fn terminated (& self , id : & BenchmarkId , context : & ReportContext)) ; reports_impl ! (fn analysis (& self , id : & BenchmarkId , context : & ReportContext)) ; reports_impl ! (fn measurement_start (& self , id : & BenchmarkId , context : & ReportContext , sample_count : u64 , estimate_ns : f64 , iter_count : u64)) ; reports_impl ! (fn measurement_complete (& self , id : & BenchmarkId , context : & ReportContext , measurements : & MeasurementData <'_ >, formatter : & dyn ValueFormatter)) ; reports_impl ! (fn summarize (& self , context : & ReportContext , all_ids : & [BenchmarkId] , formatter : & dyn ValueFormatter)) ; reports_impl ! (fn final_summary (& self , context : & ReportContext)) ; reports_impl ! (fn group_separator (& self ,)) ; }
    };
}

impl_276!()