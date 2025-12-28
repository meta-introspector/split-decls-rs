macro_rules! deps {
    () => {
        ComparisonData!();
        ReportContext!();
        MeasurementData!();
        ValueFormatter!();
        BenchmarkId!();
    };
}

macro_rules! pdf_comparison {
    () => {
        deps!();
        pub (crate) fn pdf_comparison (id : & BenchmarkId , context : & ReportContext , formatter : & dyn ValueFormatter , measurements : & MeasurementData < '_ > , comparison : & ComparisonData , size : Option < Size > ,) -> Child { let mut figure = pdf_comparison_figure (formatter , measurements , comparison , size) ; figure . set (Title (gnuplot_escape (id . as_title ()))) ; let path = context . report_path (id , "both/pdf.svg") ; debug_script (& path , & figure) ; figure . set (Output (path)) . draw () . unwrap () }
    };
}

pdf_comparison!();