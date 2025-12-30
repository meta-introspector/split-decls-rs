// Generated macro for OutParam (trait)
macro_rules! Depcrate_out_paramOutParam {
() => {
// Module: crate::out_param
// Provides: {"OutParam"}
// Dependencies: {}
# [doc = " Provides automatic parameter conversion in cases where the Windows API expects implicit conversion support."] # [doc = ""] # [doc = " This is a mutable version of [Param] meant to support out parameters."] # [doc = " There is no need to implement this trait. Blanket implementations are provided for all applicable Windows types."] pub trait OutParam < T : TypeKind , C = < T as TypeKind > :: TypeKind > : Sized where T : Type < T > , { # [doc (hidden)] unsafe fn borrow_mut (& self) -> OutRef < '_ , T > ; }
};
}
