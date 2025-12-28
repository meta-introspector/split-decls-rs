macro_rules! deps {
    () => {
        ValueFormatter!();
        MeasurementData!();
        ReportContext!();
        Data!();
        ComparisonData!();
        BenchmarkId!();
    };
}

macro_rules! regression_comparison {
    () => {
        deps!();
        pub (crate) fn regression_comparison (id : & BenchmarkId , context : & ReportContext , formatter : & dyn ValueFormatter , measurements : & MeasurementData < '_ > , comparison : & ComparisonData , base_data : & Data < '_ , f64 , f64 > , size : Option < Size > ,) -> Child { let mut figure = regression_comparison_figure (formatter , measurements , comparison , base_data , size) ; figure . set (Title (gnuplot_escape (id . as_title ()))) ; let path = context . report_path (id , "both/regression.svg") ; debug_script (& path , & figure) ; figure . set (Output (path)) . draw () . unwrap () }
    };
}

regression_comparison!()