// Generated macro for ProjectWorkspaceKind (enum)
macro_rules! Depcrate_workspaceProjectWorkspaceKind {
() => {
// Module: crate::workspace
// Provides: {"ProjectWorkspaceKind"}
// Dependencies: {}
# [derive (Clone)] # [allow (clippy :: large_enum_variant)] pub enum ProjectWorkspaceKind { # [doc = " Project workspace was discovered by running `cargo metadata` and `rustc --print sysroot`."] Cargo { # [doc = " The workspace as returned by `cargo metadata`."] cargo : CargoWorkspace , # [doc = " Additional `cargo metadata` error. (only populated if retried fetching via `--no-deps` succeeded)."] error : Option < Arc < anyhow :: Error > > , # [doc = " The build script results for the workspace."] build_scripts : WorkspaceBuildScripts , # [doc = " The rustc workspace loaded for this workspace. An `Err(None)` means loading has been"] # [doc = " disabled or was otherwise not requested."] rustc : Result < Box < (CargoWorkspace , WorkspaceBuildScripts) > , Option < String > > , } , # [doc = " Project workspace was specified using a `rust-project.json` file."] Json (ProjectJson) , # [doc = " Project with a set of disjoint files, not belonging to any particular workspace."] # [doc = " Backed by basic sysroot crates for basic completion and highlighting."] DetachedFile { # [doc = " The file in question."] file : ManifestPath , # [doc = " Is this file a cargo script file?"] cargo : Option < (CargoWorkspace , WorkspaceBuildScripts , Option < Arc < anyhow :: Error > >) > , } , }
};
}
