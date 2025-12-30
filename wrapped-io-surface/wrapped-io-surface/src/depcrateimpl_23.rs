// Generated macro for impl_23 (impl)
macro_rules! Depcrateimpl_23 {
() => {
// Module: crate
// Provides: {"impl_23"}
// Dependencies: {}
impl TCFType for IOSurface { type Ref = IOSurfaceRef ; # [inline] fn as_concrete_TypeRef (& self) -> IOSurfaceRef { self . obj } # [inline] unsafe fn wrap_under_create_rule (obj : IOSurfaceRef) -> IOSurface { assert ! (! obj . is_null () , "Attempted to create a NULL object.") ; IOSurface { obj } } # [inline] fn type_id () -> CFTypeID { unsafe { IOSurfaceGetTypeID () } } # [inline] fn as_CFTypeRef (& self) -> CFTypeRef { self . as_concrete_TypeRef () as CFTypeRef } # [inline] unsafe fn wrap_under_get_rule (reference : IOSurfaceRef) -> IOSurface { assert ! (! reference . is_null () , "Attempted to create a NULL object.") ; let reference = CFRetain (reference as * const c_void) as IOSurfaceRef ; TCFType :: wrap_under_create_rule (reference) } }
};
}
