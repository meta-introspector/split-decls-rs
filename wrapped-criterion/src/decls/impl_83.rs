macro_rules! deps {
    () => {
        BenchmarkId!();
        FileCsvReport!();
        CsvReportWriter!();
        ValueFormatter!();
        Result!();
        MeasurementData!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl FileCsvReport { fn write_file (& self , path : & Path , id : & BenchmarkId , measurements : & MeasurementData < '_ > , formatter : & dyn ValueFormatter ,) -> Result < () > { let writer = Writer :: from_path (path) ? ; let mut writer = CsvReportWriter { writer } ; writer . write_data (id , measurements , formatter) ? ; Ok (()) } }
    };
}

impl_83!()