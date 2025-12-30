// Generated macro for static_regex (macro)
macro_rules! Depcrate_utilstatic_regex {
() => {
// Module: crate::util
// Provides: {"static_regex"}
// Dependencies: {}
macro_rules ! static_regex { ($ re : literal) => { { static RE : :: std :: sync :: OnceLock <:: regex :: Regex > = :: std :: sync :: OnceLock :: new () ; RE . get_or_init (|| :: regex :: Regex :: new ($ re) . unwrap ()) } } ; }
};
}
