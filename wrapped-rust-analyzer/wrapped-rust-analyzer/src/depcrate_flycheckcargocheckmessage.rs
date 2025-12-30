// Generated macro for CargoCheckMessage (enum)
macro_rules! Depcrate_flycheckCargoCheckMessage {
() => {
// Module: crate::flycheck
// Provides: {"CargoCheckMessage"}
// Dependencies: {}
# [allow (clippy :: large_enum_variant)] enum CargoCheckMessage { CompilerArtifact (cargo_metadata :: Artifact) , Diagnostic { diagnostic : Diagnostic , package_id : Option < Arc < PackageId > > } , }
};
}
