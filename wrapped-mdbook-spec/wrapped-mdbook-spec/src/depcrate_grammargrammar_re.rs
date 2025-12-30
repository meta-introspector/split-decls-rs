// Generated macro for GRAMMAR_RE (static)
macro_rules! Depcrate_grammarGRAMMAR_RE {
() => {
// Module: crate::grammar
// Provides: {"GRAMMAR_RE"}
// Dependencies: {}
static GRAMMAR_RE : LazyLock < Regex > = LazyLock :: new (| | Regex :: new (r"(?ms)^```grammar,([^\n]+)\n(.*?)^```") . unwrap ()) ;
};
}
