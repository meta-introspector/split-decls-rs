// Generated macro for impl_240 (impl)
macro_rules! Depcrate_htmlimpl_240 {
() => {
// Module: crate::html
// Provides: {"impl_240"}
// Dependencies: {}
impl Html { pub (crate) fn new (plotter : Box < dyn Plotter >) -> Html { let mut templates = TinyTemplate :: new () ; templates . add_template ("report_link" , include_str ! ("report_link.html.tt")) . expect ("Unable to parse report_link template.") ; templates . add_template ("index" , include_str ! ("index.html.tt")) . expect ("Unable to parse index template.") ; templates . add_template ("benchmark_report" , include_str ! ("benchmark_report.html.tt")) . expect ("Unable to parse benchmark_report template") ; templates . add_template ("summary_report" , include_str ! ("summary_report.html.tt")) . expect ("Unable to parse summary_report template") ; let plotter = RefCell :: new (plotter) ; Html { templates , plotter } } }
};
}
