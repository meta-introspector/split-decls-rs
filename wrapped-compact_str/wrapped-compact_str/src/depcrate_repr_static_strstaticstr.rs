// Generated macro for StaticStr (struct)
macro_rules! Depcrate_repr_static_strStaticStr {
() => {
// Module: crate::repr::static_str
// Provides: {"StaticStr"}
// Dependencies: {}
# [doc = " A buffer stored on the stack whose size is equal to the stack size of `String`"] # [doc = " The last byte is set to 0."] # [derive (Copy , Clone)] # [repr (C)] pub (crate) struct StaticStr { ptr : ptr :: NonNull < u8 > , len : usize , # [allow (unused)] discriminant : [u8 ; DISCRIMINANT_SIZE] , }
};
}
