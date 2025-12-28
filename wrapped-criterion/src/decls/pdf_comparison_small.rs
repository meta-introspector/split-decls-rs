macro_rules! deps {
    () => {
        ComparisonData!();
        MeasurementData!();
        BenchmarkId!();
        ReportContext!();
        ValueFormatter!();
    };
}

macro_rules! pdf_comparison_small {
    () => {
        deps!();
        pub (crate) fn pdf_comparison_small (id : & BenchmarkId , context : & ReportContext , formatter : & dyn ValueFormatter , measurements : & MeasurementData < '_ > , comparison : & ComparisonData , size : Option < Size > ,) -> Child { let mut figure = pdf_comparison_figure (formatter , measurements , comparison , size) ; figure . configure (Key , | k | k . hide ()) ; let path = context . report_path (id , "relative_pdf_small.svg") ; debug_script (& path , & figure) ; figure . set (Output (path)) . draw () . unwrap () }
    };
}

pdf_comparison_small!()