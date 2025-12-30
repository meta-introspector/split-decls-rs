// Generated macro for record_bootstrap_step_durations (function)
macro_rules! Depcrate_analysisrecord_bootstrap_step_durations {
() => {
// Module: crate::analysis
// Provides: {"record_bootstrap_step_durations"}
// Dependencies: {}
fn record_bootstrap_step_durations (metrics : & JsonRoot , parent_metrics : Option < & JsonRoot >) { let parent_steps : HashMap < String , BuildStep > = parent_metrics . map (| metrics | { metrics . invocations . iter () . map (| invocation | { (invocation . cmdline . clone () , BuildStep :: from_invocation (invocation)) }) . collect () }) . unwrap_or_default () ; for invocation in & metrics . invocations { let step = BuildStep :: from_invocation (invocation) ; let table = format_build_steps (& step) ; eprintln ! ("Step `{}`\n{table}\n" , invocation . cmdline) ; output_details (& format ! ("{} (steps)" , invocation . cmdline) , | | { println ! ("<pre><code>{table}</code></pre>") ; }) ; if let Some (parent_step) = parent_steps . get (& invocation . cmdline) { let table = format_build_step_diffs (& step , parent_step) ; let duration_before = parent_step . duration . as_secs () ; let duration_after = step . duration . as_secs () ; output_details (& format ! ("{} (diff) ({duration_before}s -> {duration_after}s)" , invocation . cmdline) , | | { println ! ("{table}") ; } ,) ; } } eprintln ! ("Recorded {} bootstrap invocation(s)" , metrics . invocations . len ()) ; }
};
}
