macro_rules! deps {
    () => {
        BenchmarkId!();
        MeasurementData!();
        ValueFormatter!();
        ReportContext!();
    };
}

macro_rules! regression_small {
    () => {
        deps!();
        pub (crate) fn regression_small (id : & BenchmarkId , context : & ReportContext , formatter : & dyn ValueFormatter , measurements : & MeasurementData < '_ > , size : Option < Size > ,) -> Child { let mut figure = regression_figure (formatter , measurements , size) ; figure . configure (Key , | k | k . hide ()) ; let path = context . report_path (id , "regression_small.svg") ; debug_script (& path , & figure) ; figure . set (Output (path)) . draw () . unwrap () }
    };
}

regression_small!();