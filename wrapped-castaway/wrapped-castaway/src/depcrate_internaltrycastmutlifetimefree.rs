// Generated macro for TryCastMutLifetimeFree (trait)
macro_rules! Depcrate_internalTryCastMutLifetimeFree {
() => {
// Module: crate::internal
// Provides: {"TryCastMutLifetimeFree"}
// Dependencies: {}
# [doc = " Supporting trait for autoderef specialization on mutable references to lifetime-free"] # [doc = " types."] pub trait TryCastMutLifetimeFree < 'a , T : ? Sized , U : LifetimeFree + ? Sized > { # [inline (always)] fn try_cast (& self , value : & 'a mut T) -> Result < & 'a mut U , & 'a mut T > { if type_eq_non_static :: < T , U > () { Ok (unsafe { transmute_unchecked :: < & mut T , & mut U > (value) }) } else { Err (value) } } }
};
}
