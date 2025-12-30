// Generated macro for metadata (function)
macro_rules! Depcrate_cargometadata {
() => {
// Module: crate::cargo
// Provides: {"metadata"}
// Dependencies: {}
pub (crate) fn metadata () -> Result < Metadata > { let output = raw_cargo () . arg ("metadata") . arg ("--format-version=1") . output () . map_err (Error :: Cargo) ? ; serde_json :: from_slice (& output . stdout) . map_err (Error :: CargoMetadata) }
};
}
