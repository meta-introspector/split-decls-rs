// Generated macro for NAMES_RE (static)
macro_rules! Depcrate_grammarNAMES_RE {
() => {
// Module: crate::grammar
// Provides: {"NAMES_RE"}
// Dependencies: {}
static NAMES_RE : LazyLock < Regex > = LazyLock :: new (| | Regex :: new (r"(?m)^(?:@root )?([A-Za-z0-9_]+)(?: \([^)]+\))? ->") . unwrap ()) ;
};
}
