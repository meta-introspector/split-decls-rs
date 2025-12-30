// Generated macro for WORKSPACES (static)
macro_rules! Depcrate_envWORKSPACES {
() => {
// Module: crate::env
// Provides: {"WORKSPACES"}
// Dependencies: {}
static WORKSPACES : Lazy < Mutex < BTreeMap < String , Arc < PathBuf > > > > = Lazy :: new (| | Mutex :: new (BTreeMap :: new ())) ;
};
}
