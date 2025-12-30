// Generated macro for extract_bin (function)
macro_rules! Depcrate_testextract_bin {
() => {
// Module: crate::test
// Provides: {"extract_bin"}
// Dependencies: {}
fn extract_bin (msg : & format :: Message < '_ >) -> Option < CargoTest > { match msg { format :: Message :: CompilerArtifact (art) => { if art . profile . test { let bin_path = art . filenames . first () . expect ("files must exist") . to_path_buf () ; let kind = art . target . kind . first () . expect ("kind must exist") . as_ref () . to_owned () ; let name = art . target . name . as_ref () . to_owned () ; Some (CargoTest { bin_path , kind , name , }) } else { None } } _ => None , } }
};
}
