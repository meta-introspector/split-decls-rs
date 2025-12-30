// Generated macro for impl_65 (impl)
macro_rules! Depcrate_baseimpl_65 {
() => {
// Module: crate::base
// Provides: {"impl_65"}
// Dependencies: {}
impl TCFType for CFType { type Ref = CFTypeRef ; # [inline] fn as_concrete_TypeRef (& self) -> CFTypeRef { self . 0 } # [inline] unsafe fn wrap_under_get_rule (reference : CFTypeRef) -> CFType { assert ! (! reference . is_null () , "Attempted to create a NULL object.") ; let reference : CFTypeRef = CFRetain (reference) ; TCFType :: wrap_under_create_rule (reference) } # [inline] fn as_CFTypeRef (& self) -> CFTypeRef { self . as_concrete_TypeRef () } # [inline] unsafe fn wrap_under_create_rule (obj : CFTypeRef) -> CFType { assert ! (! obj . is_null () , "Attempted to create a NULL object.") ; CFType (obj) } # [inline] fn type_id () -> CFTypeID { 0 } }
};
}
