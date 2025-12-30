// Generated macro for CargoTargetSpec (struct)
macro_rules! Depcrate_target_specCargoTargetSpec {
() => {
// Module: crate::target_spec
// Provides: {"CargoTargetSpec"}
// Dependencies: {}
# [doc = " Abstract representation of Cargo target."] # [doc = ""] # [doc = " We use it to cook up the set of cli args we need to pass to Cargo to"] # [doc = " build/test/run the target."] # [derive (Clone , Debug)] pub (crate) struct CargoTargetSpec { pub (crate) workspace_root : AbsPathBuf , pub (crate) cargo_toml : ManifestPath , pub (crate) package : String , pub (crate) package_id : Arc < PackageId > , pub (crate) target : String , pub (crate) target_kind : TargetKind , pub (crate) crate_id : Crate , pub (crate) required_features : Vec < String > , pub (crate) features : FxHashSet < String > , pub (crate) sysroot_root : Option < vfs :: AbsPathBuf > , }
};
}
