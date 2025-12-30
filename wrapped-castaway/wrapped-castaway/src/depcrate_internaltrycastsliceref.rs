// Generated macro for TryCastSliceRef (trait)
macro_rules! Depcrate_internalTryCastSliceRef {
() => {
// Module: crate::internal
// Provides: {"TryCastSliceRef"}
// Dependencies: {}
# [doc = " Supporting trait for autoderef specialization on slices."] pub trait TryCastSliceRef < 'a , T : 'static , U : 'static > { # [doc = " Attempt to cast a generic slice to a given type if the types are equal."] # [doc = ""] # [doc = " The reference does not have to be static as long as the item type is"] # [doc = " static."] # [inline (always)] fn try_cast (& self , value : & 'a [T]) -> Result < & 'a [U] , & 'a [T] > { if type_eq :: < T , U > () { Ok (unsafe { & * (value as * const [T] as * const [U]) }) } else { Err (value) } } }
};
}
