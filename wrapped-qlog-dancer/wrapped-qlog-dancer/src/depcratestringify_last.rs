// Generated macro for stringify_last (function)
macro_rules! Depcratestringify_last {
() => {
// Module: crate
// Provides: {"stringify_last"}
// Dependencies: {}
pub fn stringify_last < T > (src : & [T]) -> String where T : std :: fmt :: Debug , { if src . len () == 1 { "n/a" . to_string () } else { format ! ("{:?}" , src . last () . unwrap ()) } }
};
}
