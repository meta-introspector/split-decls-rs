// Generated macro for TryCastRef (trait)
macro_rules! Depcrate_internalTryCastRef {
() => {
// Module: crate::internal
// Provides: {"TryCastRef"}
// Dependencies: {}
# [doc = " Supporting trait for autoderef specialization on references."] pub trait TryCastRef < 'a , T : 'static , U : 'static > { # [doc = " Attempt to cast a generic reference to a given type if the types are"] # [doc = " equal."] # [doc = ""] # [doc = " The reference does not have to be static as long as the reference target"] # [doc = " type is static."] # [inline (always)] fn try_cast (& self , value : & 'a T) -> Result < & 'a U , & 'a T > { if type_eq :: < T , U > () { Ok (unsafe { & * (value as * const T as * const U) }) } else { Err (value) } } }
};
}
