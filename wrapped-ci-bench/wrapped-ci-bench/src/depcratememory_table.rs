// Generated macro for memory_table (function)
macro_rules! Depcratememory_table {
() => {
// Module: crate
// Provides: {"memory_table"}
// Dependencies: {}
# [doc = " Renders the diffs as a markdown table"] fn memory_table (diffs : & [MemoryDiff] , emoji_feedback : bool) { println ! ("| Scenario | Baseline | Candidate | Diff |") ; println ! ("| --- | ---: | ---: | ---: |") ; for diff in diffs { let emoji = match emoji_feedback { true if diff . diff_ratio > 0.01 => "⚠️ " , true if diff . diff_ratio < - 0.01 => "✅ " , _ => "" , } ; println ! ("| {} | Total {}B / {}# <br/> Peak {}B / {}# | Total {}B / {}# <br/> Peak {}B / {}# | {:?} {}{} ({:.2}%) |" , diff . scenario , diff . baseline . heap_total_bytes , diff . baseline . heap_total_blocks , diff . baseline . heap_peak_bytes , diff . baseline . heap_peak_blocks , diff . candidate . heap_total_bytes , diff . candidate . heap_total_blocks , diff . candidate . heap_peak_bytes , diff . candidate . heap_peak_blocks , diff . comparator , emoji , diff . diff , diff . diff_ratio * 100.0) } }
};
}
