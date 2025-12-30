// Generated macro for FlycheckScope (enum)
macro_rules! Depcrate_flycheckFlycheckScope {
() => {
// Module: crate::flycheck
// Provides: {"FlycheckScope"}
// Dependencies: {}
enum FlycheckScope { Workspace , Package { package : Arc < PackageId > , workspace_deps : Option < FxHashSet < Arc < PackageId > > > } , }
};
}
