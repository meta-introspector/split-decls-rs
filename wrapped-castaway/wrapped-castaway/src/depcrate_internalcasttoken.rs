// Generated macro for CastToken (struct)
macro_rules! Depcrate_internalCastToken {
() => {
// Module: crate::internal
// Provides: {"CastToken"}
// Dependencies: {}
# [doc = " A token struct used to capture a type without taking ownership of any"] # [doc = " values. Used to select a cast implementation in macros."] pub struct CastToken < T : ? Sized > (PhantomData < T >) ;
};
}
