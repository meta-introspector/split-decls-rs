// Generated macro for TOOL_CONFIGS (static)
macro_rules! Depcrate_envTOOL_CONFIGS {
() => {
// Module: crate::env
// Provides: {"TOOL_CONFIGS"}
// Dependencies: {}
static TOOL_CONFIGS : Lazy < Mutex < BTreeMap < PathBuf , Arc < ToolConfig > > > > = Lazy :: new (| | Mutex :: new (BTreeMap :: new ())) ;
};
}
