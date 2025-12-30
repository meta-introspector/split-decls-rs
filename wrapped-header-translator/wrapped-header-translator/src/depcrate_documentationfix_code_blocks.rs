// Generated macro for fix_code_blocks (function)
macro_rules! Depcrate_documentationfix_code_blocks {
() => {
// Module: crate::documentation
// Provides: {"fix_code_blocks"}
// Dependencies: {}
fn fix_code_blocks (s : & str) -> String { let mut ret = String :: with_capacity (s . len ()) ; let mut last_end = 0 ; for (i , (start , part)) in s . match_indices ("```") . enumerate () { if i % 2 == 1 { continue ; } if & s [start .. start + 4] != "```\n" { continue ; } ret . push_str (& s [last_end .. start]) ; ret . push_str ("```text") ; last_end = start + part . len () ; } ret . push_str (& s [last_end .. s . len ()]) ; ret }
};
}
