// Generated macro for WorkspaceBuildScripts (struct)
macro_rules! Depcrate_build_dependenciesWorkspaceBuildScripts {
() => {
// Module: crate::build_dependencies
// Provides: {"WorkspaceBuildScripts"}
// Dependencies: {}
# [doc = " Output of the build script and proc-macro building steps for a workspace."] # [derive (Debug , Default , Clone , PartialEq , Eq)] pub struct WorkspaceBuildScripts { outputs : ArenaMap < Package , BuildScriptOutput > , error : Option < String > , }
};
}
