// Generated macro for indent (function)
macro_rules! Depcrate_world_generatorindent {
() => {
// Module: crate::world_generator
// Provides: {"indent"}
// Dependencies: {}
fn indent (code : & str) -> String { let mut indented = String :: with_capacity (code . len ()) ; let mut indent = 0 ; let mut was_empty = false ; for line in code . trim () . lines () { let trimmed = line . trim () ; if trimmed . is_empty () { if was_empty { continue ; } was_empty = true ; } else { was_empty = false ; } if trimmed . starts_with ('}') { indent -= 1 ; } if ! trimmed . is_empty () { indented . extend (iter :: repeat (' ') . take (indent * 4)) ; indented . push_str (trimmed) ; } if trimmed . ends_with ('{') { indent += 1 ; } indented . push ('\n') ; } indented }
};
}
