// Generated macro for get_target_spec_from_msrv (function)
macro_rules! Depcrate_readget_target_spec_from_msrv {
() => {
// Module: crate::read
// Provides: {"get_target_spec_from_msrv"}
// Dependencies: {}
pub fn get_target_spec_from_msrv (target : & str) -> TargetSpec { let mut cmd = process :: Command :: new ("rustc") ; cmd . args (["+1.63" , "-Zunstable-options" , "--print" , "target-spec-json" , "--target" , target ,]) ; cmd . env ("RUSTC_BOOTSTRAP" , "1") ; cmd . stdout (process :: Stdio :: piped ()) ; cmd . stderr (process :: Stdio :: inherit ()) ; let process :: Output { status , stdout , .. } = cmd . output () . unwrap () ; if ! status . success () { panic ! ("{:?} failed with non-zero exit status: {}" , cmd , status) } serde_json :: from_slice (& stdout) . unwrap () }
};
}
