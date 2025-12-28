macro_rules! deps {
    () => {
        ValueFormatter!();
        MeasurementData!();
        ComparisonData!();
        ReportContext!();
        BenchmarkId!();
    };
}

macro_rules! iteration_times_comparison_small {
    () => {
        deps!();
        pub (crate) fn iteration_times_comparison_small (id : & BenchmarkId , context : & ReportContext , formatter : & dyn ValueFormatter , measurements : & MeasurementData < '_ > , comparison : & ComparisonData , size : Option < Size > ,) -> Child { let mut figure = iteration_times_comparison_figure (formatter , measurements , comparison , size) ; figure . configure (Key , | k | k . hide ()) ; let path = context . report_path (id , "relative_iteration_times_small.svg") ; debug_script (& path , & figure) ; figure . set (Output (path)) . draw () . unwrap () }
    };
}

iteration_times_comparison_small!()