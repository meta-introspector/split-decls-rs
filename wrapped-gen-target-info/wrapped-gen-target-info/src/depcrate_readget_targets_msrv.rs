// Generated macro for get_targets_msrv (function)
macro_rules! Depcrate_readget_targets_msrv {
() => {
// Module: crate::read
// Provides: {"get_targets_msrv"}
// Dependencies: {}
pub fn get_targets_msrv () -> Vec < u8 > { let mut cmd = process :: Command :: new ("rustc") ; cmd . args (["+1.63" , "--print" , "target-list"]) ; cmd . stdout (process :: Stdio :: piped ()) ; cmd . stderr (process :: Stdio :: inherit ()) ; let process :: Output { status , stdout , .. } = cmd . output () . unwrap () ; if ! status . success () { panic ! ("{:?} failed with non-zero exit status: {}" , cmd , status) } stdout }
};
}
