// Generated macro for TryCastSliceMut (trait)
macro_rules! Depcrate_internalTryCastSliceMut {
() => {
// Module: crate::internal
// Provides: {"TryCastSliceMut"}
// Dependencies: {}
# [doc = " Supporting trait for autoderef specialization on mutable slices."] pub trait TryCastSliceMut < 'a , T : 'static , U : 'static > { # [doc = " Attempt to cast a generic mutable slice to a given type if the types are"] # [doc = " equal."] # [doc = ""] # [doc = " The reference does not have to be static as long as the item type is"] # [doc = " static."] # [inline (always)] fn try_cast (& self , value : & 'a mut [T]) -> Result < & 'a mut [U] , & 'a mut [T] > { if type_eq :: < T , U > () { Ok (unsafe { & mut * (value as * mut [T] as * mut [U]) }) } else { Err (value) } } }
};
}
