// Generated macro for get_target_specs_from_json (function)
macro_rules! Depcrate_readget_target_specs_from_json {
() => {
// Module: crate::read
// Provides: {"get_target_specs_from_json"}
// Dependencies: {}
pub fn get_target_specs_from_json (rustc : Option < String >) -> RustcTargetSpecs { let mut cmd = process :: Command :: new (rustc . clone () . unwrap_or ("rustc" . into ())) ; if rustc . is_none () { cmd . arg ("+nightly") ; } cmd . args (["-Zunstable-options" , "--print" , "all-target-specs-json"]) ; cmd . stdout (process :: Stdio :: piped ()) ; cmd . stderr (process :: Stdio :: inherit ()) ; let process :: Output { status , stdout , .. } = cmd . output () . unwrap () ; if ! status . success () { panic ! ("{:?} failed with non-zero exit status: {}" , cmd , status) } serde_json :: from_slice (& stdout) . unwrap () }
};
}
