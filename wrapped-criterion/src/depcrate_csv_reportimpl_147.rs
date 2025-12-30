// Generated macro for impl_147 (impl)
macro_rules! Depcrate_csv_reportimpl_147 {
() => {
// Module: crate::csv_report
// Provides: {"impl_147"}
// Dependencies: {}
impl Report for FileCsvReport { fn measurement_complete (& self , id : & BenchmarkId , context : & ReportContext , measurements : & MeasurementData < '_ > , formatter : & dyn ValueFormatter ,) { let mut path = context . output_directory . clone () ; path . push (id . as_directory_name ()) ; path . push ("new") ; path . push ("raw.csv") ; log_if_err ! (self . write_file (& path , id , measurements , formatter)) ; } }
};
}
