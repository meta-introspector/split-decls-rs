// Generated macro for post_merge_report (function)
macro_rules! Depcratepost_merge_report {
() => {
// Module: crate
// Provides: {"post_merge_report"}
// Dependencies: {}
fn post_merge_report (db : JobDatabase , current : String , parent : String) -> anyhow :: Result < () > { let metrics = download_auto_job_metrics (& db , Some (& parent) , & current) ? ; println ! ("\nComparing {parent} (parent) -> {current} (this PR)\n") ; let mut job_info_resolver = JobInfoResolver :: new () ; output_test_diffs (& metrics , & mut job_info_resolver) ; output_details ("Test dashboard" , | | { println ! (r#"Run

```bash
cargo run --manifest-path src/ci/citool/Cargo.toml -- \
    test-dashboard {current} --output-dir test-dashboard
```
And then open `test-dashboard/index.html` in your browser to see an overview of all executed tests.
"#) ; }) ; output_largest_duration_changes (& metrics , & mut job_info_resolver) ; Ok (()) }
};
}
