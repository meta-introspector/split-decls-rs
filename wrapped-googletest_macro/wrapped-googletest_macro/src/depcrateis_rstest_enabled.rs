// Generated macro for is_rstest_enabled (function)
macro_rules! Depcrateis_rstest_enabled {
() => {
// Module: crate
// Provides: {"is_rstest_enabled"}
// Dependencies: {}
fn is_rstest_enabled (attributes : & [Attribute]) -> bool { attributes . iter () . any (| attr | matches ! (attr . path () . segments . last () , Some (last_segment) if last_segment . ident == "rstest")) }
};
}
