macro_rules! deps {
    () => {
        ValueFormatter!();
        MeasurementData!();
        ReportContext!();
        ComparisonData!();
        BenchmarkId!();
    };
}

macro_rules! iteration_times_comparison {
    () => {
        deps!();
        pub (crate) fn iteration_times_comparison (id : & BenchmarkId , context : & ReportContext , formatter : & dyn ValueFormatter , measurements : & MeasurementData < '_ > , comparison : & ComparisonData , size : Option < Size > ,) -> Child { let mut figure = iteration_times_comparison_figure (formatter , measurements , comparison , size) ; figure . set (Title (gnuplot_escape (id . as_title ()))) ; let path = context . report_path (id , "both/iteration_times.svg") ; debug_script (& path , & figure) ; figure . set (Output (path)) . draw () . unwrap () }
    };
}

iteration_times_comparison!();