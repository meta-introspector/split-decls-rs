// Generated macro for output_largest_duration_changes (function)
macro_rules! Depcrate_analysisoutput_largest_duration_changes {
() => {
// Module: crate::analysis
// Provides: {"output_largest_duration_changes"}
// Dependencies: {}
# [doc = " Prints the ten largest differences in bootstrap durations."] pub fn output_largest_duration_changes (job_metrics : & HashMap < JobName , JobMetrics > , job_info_resolver : & mut JobInfoResolver ,) { struct Entry < 'a > { job : & 'a JobName , before : Duration , after : Duration , change : f64 , } let mut changes : Vec < Entry > = vec ! [] ; for (job , metrics) in job_metrics { if let Some (parent) = & metrics . parent { let duration_before = parent . invocations . iter () . map (| i | BuildStep :: from_invocation (i) . duration) . sum :: < Duration > () ; let duration_after = metrics . current . invocations . iter () . map (| i | BuildStep :: from_invocation (i) . duration) . sum :: < Duration > () ; let pct_change = duration_after . as_secs_f64 () / duration_before . as_secs_f64 () ; let pct_change = pct_change * 100.0 ; let pct_change = pct_change - 100.0 ; changes . push (Entry { job , before : duration_before , after : duration_after , change : pct_change , }) ; } } changes . sort_by (| e1 , e2 | e1 . change . abs () . partial_cmp (& e2 . change . abs ()) . unwrap () . reverse ()) ; println ! ("# Job duration changes") ; for (index , entry) in changes . into_iter () . take (10) . enumerate () { println ! ("{}. {}: {:.1}s -> {:.1}s ({:.1}%)" , index + 1 , format_job_link (job_info_resolver , job_metrics , entry . job) , entry . before . as_secs_f64 () , entry . after . as_secs_f64 () , entry . change) ; } println ! () ; output_details ("How to interpret the job duration changes?" , | | { println ! (r#"Job durations can vary a lot, based on the actual runner instance
that executed the job, system noise, invalidated caches, etc. The table above is provided
mostly for t-infra members, for simpler debugging of potential CI slow-downs."#) ; }) ; }
};
}
