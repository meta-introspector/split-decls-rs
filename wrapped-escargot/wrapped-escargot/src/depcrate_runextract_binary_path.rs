// Generated macro for extract_binary_path (function)
macro_rules! Depcrate_runextract_binary_path {
() => {
// Module: crate::run
// Provides: {"extract_binary_path"}
// Dependencies: {}
fn extract_binary_path (msgs : CommandMessages , kind : & 'static str ,) -> Result < path :: PathBuf , CargoError > { let bins : Result < Vec < _ > , CargoError > = extract_binary_paths (msgs , kind) . collect () ; let bins = bins ? ; if bins . is_empty () { return Err (CargoError :: new (ErrorKind :: CommandFailed) . set_context ("No binaries in crate")) ; } else if bins . len () != 1 { return Err (CargoError :: new (ErrorKind :: CommandFailed) . set_context (std :: format ! ("Ambiguous which binary is intended: {bins:?}"))) ; } Ok (bins . into_iter () . next () . expect ("already validated")) }
};
}
