// Generated macro for TryCastOwned (trait)
macro_rules! Depcrate_internalTryCastOwned {
() => {
// Module: crate::internal
// Provides: {"TryCastOwned"}
// Dependencies: {}
# [doc = " Default trait for autoderef specialization."] pub trait TryCastOwned < T : 'static , U : 'static > { # [doc = " Attempt to cast a value to a given type if the types are equal."] # [inline (always)] fn try_cast (& self , value : T) -> Result < U , T > { if type_eq :: < T , U > () { Ok (unsafe { transmute_unchecked :: < T , U > (value) }) } else { Err (value) } } }
};
}
