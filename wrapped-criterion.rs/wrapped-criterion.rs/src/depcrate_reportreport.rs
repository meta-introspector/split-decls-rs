// Generated macro for Report (trait)
macro_rules! Depcrate_reportReport {
() => {
// Module: crate::report
// Provides: {"Report"}
// Dependencies: {}
pub (crate) trait Report { fn test_start (& self , _id : & BenchmarkId , _context : & ReportContext) { } fn test_pass (& self , _id : & BenchmarkId , _context : & ReportContext) { } fn benchmark_start (& self , _id : & BenchmarkId , _context : & ReportContext) { } fn profile (& self , _id : & BenchmarkId , _context : & ReportContext , _profile_ns : f64) { } fn warmup (& self , _id : & BenchmarkId , _context : & ReportContext , _warmup_ns : f64) { } fn terminated (& self , _id : & BenchmarkId , _context : & ReportContext) { } fn analysis (& self , _id : & BenchmarkId , _context : & ReportContext) { } fn measurement_start (& self , _id : & BenchmarkId , _context : & ReportContext , _sample_count : u64 , _estimate_ns : f64 , _iter_count : u64 ,) { } fn measurement_complete (& self , _id : & BenchmarkId , _context : & ReportContext , _measurements : & MeasurementData < '_ > , _formatter : & dyn ValueFormatter ,) { } fn summarize (& self , _context : & ReportContext , _all_ids : & [BenchmarkId] , _formatter : & dyn ValueFormatter ,) { } fn final_summary (& self , _context : & ReportContext) { } fn group_separator (& self) { } }
};
}
