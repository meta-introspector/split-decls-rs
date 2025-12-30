// Generated macro for make_chart_config (function)
macro_rules! Depcrate_plotsmake_chart_config {
() => {
// Module: crate::plots
// Provides: {"make_chart_config"}
// Dependencies: {}
fn make_chart_config (title : & str , params : & PlotParameters , filename : & str , ds : & Datastore , ty : & ChartOutputType ,) -> ChartConfig { let chart_config = ChartConfig { title : title . into () , input_filename : filename . into () , clamp : params . clamp . clone () , app_proto : ds . application_proto , host : ds . host . clone () , session_id : ds . session_id , ty : ty . clone () , } ; chart_config . init_chart_dir () ; chart_config }
};
}
