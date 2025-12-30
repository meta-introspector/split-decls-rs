// Generated macro for version (function)
macro_rules! Depcrate_renderversion {
() => {
// Module: crate::render
// Provides: {"version"}
// Dependencies: {}
pub (crate) fn version (cmd : & clap :: Command) -> String { format ! ("v{}" , cmd . get_long_version () . or_else (|| cmd . get_version ()) . unwrap ()) }
};
}
