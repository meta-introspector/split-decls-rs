// Generated macro for random (function)
macro_rules! Depcrate_swayrandom {
() => {
// Module: crate::sway
// Provides: {"random"}
// Dependencies: {}
fn random (rng : & mut impl Rng) -> FTXresponse < Trade > { if rng . gen () { FTXresponse :: Result (FTXresponseSuccess { result : Trade :: random (rng) , success : rng . gen () , }) } else { FTXresponse :: Error (FTXresponseFailure { success : rng . gen () , error : crate :: gen_string (rng) , }) } }
};
}
