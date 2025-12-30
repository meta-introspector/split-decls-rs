// Generated macro for tests (module)
macro_rules! Depcrate_runtime_ivartests {
() => {
// Module: crate::runtime::ivar
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use crate :: class ; use crate :: runtime :: { test_utils , AnyClass } ; fn get_ivar_layout (cls : & AnyClass) -> * const u8 { unsafe { ffi :: class_getIvarLayout (cls) } } # [test] # [cfg_attr (feature = "gnustep-1-7" , ignore = "ivar layout is still used on GNUStep")] fn test_layout_does_not_matter_any_longer () { assert ! (get_ivar_layout (class ! (NSObject)) . is_null ()) ; assert ! (get_ivar_layout (class ! (NSArray)) . is_null ()) ; assert ! (get_ivar_layout (class ! (NSException)) . is_null ()) ; assert ! (get_ivar_layout (class ! (NSNumber)) . is_null ()) ; assert ! (get_ivar_layout (class ! (NSString)) . is_null ()) ; } # [test] # [cfg_attr (all (debug_assertions , not (feature = "disable-encoding-assertions")) , should_panic = "wrong encoding. Tried to retrieve ivar with encoding I, but the encoding of the given type was C")] fn test_object_ivar_wrong_type () { let obj = test_utils :: custom_object () ; let cls = test_utils :: custom_class () ; let ivar = cls . instance_variable (CStr :: from_bytes_with_nul (b"_foo\0") . unwrap ()) . unwrap () ; let _ = unsafe { * ivar . load :: < u8 > (& obj) } ; } }
};
}
