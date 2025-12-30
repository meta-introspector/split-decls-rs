// Generated macro for summarize_and_print_changes (function)
macro_rules! Depcrate_outputsummarize_and_print_changes {
() => {
// Module: crate::output
// Provides: {"summarize_and_print_changes"}
// Dependencies: {}
# [doc = " Creates the log file output for [`OutputFormat::Text`] and [`OutputFormat::Markdown`]"] pub fn summarize_and_print_changes (warnings : & [ClippyWarning] , ices : & [RustcIce] , clippy_ver : String , config : & LintcheckConfig ,) -> String { let (stats_formatted , new_stats) = gather_stats (warnings) ; let old_stats = read_stats_from_file (& config . lintcheck_results_path) ; let mut all_msgs : Vec < String > = warnings . iter () . map (| warn | warn . to_output (config . format)) . collect () ; all_msgs . sort () ; all_msgs . push ("\n\n### Stats:\n\n" . into ()) ; all_msgs . push (stats_formatted) ; let mut text = clippy_ver ; text . push_str ("\n### Reports\n\n") ; if config . format == OutputFormat :: Markdown { text . push_str ("| file | lint | message |\n") ; text . push_str ("| --- | --- | --- |\n") ; } write ! (text , "{}" , all_msgs . join ("")) . unwrap () ; text . push_str ("\n\n### ICEs:\n") ; for ice in ices { writeln ! (text , "{ice}") . unwrap () ; } print_stats (old_stats , new_stats , & config . lint_filter) ; text }
};
}
