// Generated macro for table (function)
macro_rules! Depcratetable {
() => {
// Module: crate
// Provides: {"table"}
// Dependencies: {}
# [doc = " Renders the diffs as a markdown table"] fn table < 'a > (diffs : impl Iterator < Item = & 'a Diff > , emoji_feedback : bool) { println ! ("| Scenario | Baseline | Candidate | Diff |") ; println ! ("| --- | ---: | ---: | ---: |") ; for diff in diffs { let emoji = match emoji_feedback { true if diff . diff > 0 => "⚠️ " , true if diff . diff < 0 => "✅ " , _ => "" , } ; println ! ("| {} | {} | {} | {}{} ({:.2}%) |" , diff . scenario , diff . baseline , diff . candidate , emoji , diff . diff , diff . diff_ratio * 100.0) } }
};
}
