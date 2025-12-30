// Generated macro for TryCastMut (trait)
macro_rules! Depcrate_internalTryCastMut {
() => {
// Module: crate::internal
// Provides: {"TryCastMut"}
// Dependencies: {}
# [doc = " Supporting trait for autoderef specialization on mutable references."] pub trait TryCastMut < 'a , T : 'static , U : 'static > { # [doc = " Attempt to cast a generic mutable reference to a given type if the types"] # [doc = " are equal."] # [doc = ""] # [doc = " The reference does not have to be static as long as the reference target"] # [doc = " type is static."] # [inline (always)] fn try_cast (& self , value : & 'a mut T) -> Result < & 'a mut U , & 'a mut T > { if type_eq :: < T , U > () { Ok (unsafe { & mut * (value as * mut T as * mut U) }) } else { Err (value) } } }
};
}
