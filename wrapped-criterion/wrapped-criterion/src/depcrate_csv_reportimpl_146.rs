// Generated macro for impl_146 (impl)
macro_rules! Depcrate_csv_reportimpl_146 {
() => {
// Module: crate::csv_report
// Provides: {"impl_146"}
// Dependencies: {}
impl FileCsvReport { fn write_file (& self , path : & Path , id : & BenchmarkId , measurements : & MeasurementData < '_ > , formatter : & dyn ValueFormatter ,) -> Result < () > { let writer = Writer :: from_path (path) ? ; let mut writer = CsvReportWriter { writer } ; writer . write_data (id , measurements , formatter) ? ; Ok (()) } }
};
}
