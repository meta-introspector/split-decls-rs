// Generated macro for impl_62 (impl)
macro_rules! Depcrate_benchmark_groupimpl_62 {
() => {
// Module: crate::benchmark_group
// Provides: {"impl_62"}
// Dependencies: {}
impl < 'a , M : Measurement > Drop for BenchmarkGroup < 'a , M > { fn drop (& mut self) { if let Some (conn) = & mut self . criterion . connection { conn . send (& OutgoingMessage :: FinishedBenchmarkGroup { group : & self . group_name , }) . unwrap () ; conn . serve_value_formatter (self . criterion . measurement . formatter ()) . unwrap () ; } if self . all_ids . len () > 1 && self . any_matched && self . criterion . mode . is_benchmark () { let report_context = ReportContext { output_directory : self . criterion . output_directory . clone () , plot_config : self . partial_config . plot_config . clone () , } ; self . criterion . report . summarize (& report_context , & self . all_ids , self . criterion . measurement . formatter () ,) ; } if self . any_matched && ! self . criterion . mode . is_terse () { self . criterion . report . group_separator () ; } } }
};
}
