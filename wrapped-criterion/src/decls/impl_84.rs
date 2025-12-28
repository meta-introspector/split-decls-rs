macro_rules! deps {
    () => {
        ReportContext!();
        BenchmarkId!();
        MeasurementData!();
        FileCsvReport!();
        ValueFormatter!();
        Report!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl Report for FileCsvReport { fn measurement_complete (& self , id : & BenchmarkId , context : & ReportContext , measurements : & MeasurementData < '_ > , formatter : & dyn ValueFormatter ,) { let mut path = context . output_directory . clone () ; path . push (id . as_directory_name ()) ; path . push ("new") ; path . push ("raw.csv") ; log_if_err ! (self . write_file (& path , id , measurements , formatter)) ; } }
    };
}

impl_84!();