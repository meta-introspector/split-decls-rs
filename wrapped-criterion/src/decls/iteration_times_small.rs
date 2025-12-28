macro_rules! deps {
    () => {
        MeasurementData!();
        BenchmarkId!();
        ReportContext!();
        ValueFormatter!();
    };
}

macro_rules! iteration_times_small {
    () => {
        deps!();
        pub (crate) fn iteration_times_small (id : & BenchmarkId , context : & ReportContext , formatter : & dyn ValueFormatter , measurements : & MeasurementData < '_ > , size : Option < Size > ,) -> Child { let mut figure = iteration_times_figure (formatter , measurements , size) ; figure . configure (Key , | k | k . hide ()) ; let path = context . report_path (id , "iteration_times_small.svg") ; debug_script (& path , & figure) ; figure . set (Output (path)) . draw () . unwrap () }
    };
}

iteration_times_small!()