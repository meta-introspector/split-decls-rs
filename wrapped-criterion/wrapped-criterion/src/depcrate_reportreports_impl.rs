// Generated macro for reports_impl (macro)
macro_rules! Depcrate_reportreports_impl {
() => {
// Module: crate::report
// Provides: {"reports_impl"}
// Dependencies: {}
macro_rules ! reports_impl { (fn $ name : ident (& self , $ ($ argn : ident : $ argt : ty) ,*)) => { fn $ name (& self , $ ($ argn : $ argt) ,*) { if self . cli_enabled { self . cli .$ name ($ ($ argn) ,*) ; } if self . bencher_enabled { self . bencher .$ name ($ ($ argn) ,*) ; } # [cfg (feature = "csv_output")] if self . csv_enabled { FileCsvReport .$ name ($ ($ argn) ,*) ; } if let Some (reporter) = & self . html { reporter .$ name ($ ($ argn) ,*) ; } } } ; }
};
}
