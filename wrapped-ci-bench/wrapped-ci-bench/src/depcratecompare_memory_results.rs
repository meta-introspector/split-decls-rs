// Generated macro for compare_memory_results (function)
macro_rules! Depcratecompare_memory_results {
() => {
// Module: crate
// Provides: {"compare_memory_results"}
// Dependencies: {}
# [doc = " Returns an internal representation of the comparison between the baseline and the candidate"] # [doc = " measurements"] fn compare_memory_results (baseline : & HashMap < String , MemoryDetails > , candidate : & HashMap < String , MemoryDetails > , comparator : CompareMemoryOperand ,) -> anyhow :: Result < MemoryCompareResult > { let mut diffs = Vec :: new () ; let mut missing = Vec :: new () ; for (scenario , & candidate_memory) in candidate { let Some (& baseline_memory) = baseline . get (scenario) else { missing . push (scenario . clone ()) ; continue ; } ; let candidate_count = comparator . choose (candidate_memory) ; let baseline_count = comparator . choose (baseline_memory) ; let diff = candidate_count as i64 - baseline_count as i64 ; let diff_ratio = diff as f64 / baseline_count as f64 ; let diff = MemoryDiff { scenario : scenario . clone () , baseline : baseline_memory , candidate : candidate_memory , comparator , diff , diff_ratio , } ; diffs . push (diff) ; } diffs . sort_by (| diff1 , diff2 | { diff2 . diff_ratio . abs () . total_cmp (& diff1 . diff_ratio . abs ()) }) ; Ok (MemoryCompareResult { diffs , missing_in_baseline : missing , }) }
};
}
