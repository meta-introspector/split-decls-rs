// Generated macro for impl_8 (impl)
macro_rules! Depcrate_errorimpl_8 {
() => {
// Module: crate::error
// Provides: {"impl_8"}
// Dependencies: {}
impl TryFrom < u32 > for FeatureGateError { type Error = ProgramError ; fn try_from (error : u32) -> Result < Self , Self :: Error > { match error { 0 => Ok (FeatureGateError :: FeatureAlreadyActivated) , _ => Err (ProgramError :: InvalidArgument) , } } }
};
}
