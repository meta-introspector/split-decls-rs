// Generated macro for uwtable_attr (function)
macro_rules! Depcrate_attributesuwtable_attr {
() => {
// Module: crate::attributes
// Provides: {"uwtable_attr"}
// Dependencies: {}
# [doc = " Tell LLVM to emit or not emit the information necessary to unwind the stack for the function."] # [inline] pub (crate) fn uwtable_attr (llcx : & llvm :: Context , use_sync_unwind : Option < bool >) -> & Attribute { let async_unwind = ! use_sync_unwind . unwrap_or (false) ; llvm :: CreateUWTableAttr (llcx , async_unwind) }
};
}
