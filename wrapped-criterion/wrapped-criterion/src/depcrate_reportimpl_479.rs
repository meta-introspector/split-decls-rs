// Generated macro for impl_479 (impl)
macro_rules! Depcrate_reportimpl_479 {
() => {
// Module: crate::report
// Provides: {"impl_479"}
// Dependencies: {}
impl ReportContext { pub fn report_path < P : AsRef < Path > + ? Sized > (& self , id : & BenchmarkId , file_name : & P) -> PathBuf { let mut path = self . output_directory . clone () ; path . push (id . as_directory_name ()) ; path . push ("report") ; path . push (file_name) ; path } }
};
}
