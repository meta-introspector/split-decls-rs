// Generated macro for impl_92 (impl)
macro_rules! Depcrate_cargo_workspaceimpl_92 {
() => {
// Module: crate::cargo_workspace
// Provides: {"impl_92"}
// Dependencies: {}
impl DepKind { fn iter (list : & [cargo_metadata :: DepKindInfo]) -> impl Iterator < Item = Self > { let mut dep_kinds = [None ; 3] ; if list . is_empty () { dep_kinds [0] = Some (Self :: Normal) ; } for info in list { match info . kind { cargo_metadata :: DependencyKind :: Normal => dep_kinds [0] = Some (Self :: Normal) , cargo_metadata :: DependencyKind :: Development => dep_kinds [1] = Some (Self :: Dev) , cargo_metadata :: DependencyKind :: Build => dep_kinds [2] = Some (Self :: Build) , cargo_metadata :: DependencyKind :: Unknown => continue , } } dep_kinds . into_iter () . flatten () } }
};
}
