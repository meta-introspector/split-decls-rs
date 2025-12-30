// Generated macro for TryCastRefLifetimeFree (trait)
macro_rules! Depcrate_internalTryCastRefLifetimeFree {
() => {
// Module: crate::internal
// Provides: {"TryCastRefLifetimeFree"}
// Dependencies: {}
# [doc = " Supporting trait for autoderef specialization on references to lifetime-free"] # [doc = " types."] pub trait TryCastRefLifetimeFree < 'a , T : ? Sized , U : LifetimeFree + ? Sized > { # [inline (always)] fn try_cast (& self , value : & 'a T) -> Result < & 'a U , & 'a T > { if type_eq_non_static :: < T , U > () { Ok (unsafe { transmute_unchecked :: < & T , & U > (value) }) } else { Err (value) } } }
};
}
