// Generated macro for compare_icount_results (function)
macro_rules! Depcratecompare_icount_results {
() => {
// Module: crate
// Provides: {"compare_icount_results"}
// Dependencies: {}
# [doc = " Returns an internal representation of the comparison between the baseline and the candidate"] # [doc = " measurements"] fn compare_icount_results (baseline_dir : & Path , candidate_dir : & Path , baseline : & HashMap < String , u64 > , candidate : & HashMap < String , u64 > ,) -> anyhow :: Result < CompareResult > { let mut diffs = Vec :: new () ; let mut missing = Vec :: new () ; for (scenario , & instr_count) in candidate { let Some (& baseline_instr_count) = baseline . get (scenario) else { missing . push (scenario . clone ()) ; continue ; } ; let diff = instr_count as i64 - baseline_instr_count as i64 ; let diff_ratio = diff as f64 / baseline_instr_count as f64 ; let diff = Diff { scenario : scenario . clone () , baseline : baseline_instr_count , candidate : instr_count , diff , diff_ratio , } ; diffs . push (diff) ; } diffs . sort_by (| diff1 , diff2 | { diff2 . diff_ratio . abs () . total_cmp (& diff1 . diff_ratio . abs ()) }) ; let mut diffs_with_callgrind_diff = Vec :: new () ; for diff in diffs { let detailed_diff = valgrind :: callgrind_diff (baseline_dir , candidate_dir , & diff . scenario) ? ; diffs_with_callgrind_diff . push ((diff , detailed_diff)) ; } Ok (CompareResult { diffs : diffs_with_callgrind_diff , missing_in_baseline : missing , }) }
};
}
