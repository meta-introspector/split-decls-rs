// Generated macro for TryCastOwnedLifetimeFree (trait)
macro_rules! Depcrate_internalTryCastOwnedLifetimeFree {
() => {
// Module: crate::internal
// Provides: {"TryCastOwnedLifetimeFree"}
// Dependencies: {}
# [doc = " Supporting trait for autoderef specialization on lifetime-free types."] pub trait TryCastOwnedLifetimeFree < T , U : LifetimeFree > { # [inline (always)] fn try_cast (& self , value : T) -> Result < U , T > { if type_eq_non_static :: < T , U > () { Ok (unsafe { transmute_unchecked :: < T , U > (value) }) } else { Err (value) } } }
};
}
