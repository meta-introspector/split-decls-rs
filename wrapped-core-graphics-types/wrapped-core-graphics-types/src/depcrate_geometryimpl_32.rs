// Generated macro for impl_32 (impl)
macro_rules! Depcrate_geometryimpl_32 {
() => {
// Module: crate::geometry
// Provides: {"impl_32"}
// Dependencies: {}
impl CGRect { # [inline] pub fn new (origin : & CGPoint , size : & CGSize) -> CGRect { CGRect { origin : * origin , size : * size , } } # [inline] pub fn inset (& self , size : & CGSize) -> CGRect { unsafe { ffi :: CGRectInset (* self , size . width , size . height) } } # [inline] pub fn from_dict_representation (dict : & CFDictionary) -> Option < CGRect > { let mut rect = CGRect :: new (& CGPoint :: new (0. , 0.) , & CGSize :: new (0. , 0.)) ; let result = unsafe { ffi :: CGRectMakeWithDictionaryRepresentation (dict . as_concrete_TypeRef () , & mut rect) } ; if result == 0 { None } else { Some (rect) } } # [inline] pub fn is_empty (& self) -> bool { unsafe { ffi :: CGRectIsEmpty (* self) == 1 } } # [inline] pub fn is_intersects (& self , other : & CGRect) -> bool { unsafe { ffi :: CGRectIntersectsRect (* self , * other) == 1 } } # [inline] pub fn apply_transform (& self , t : & CGAffineTransform) -> CGRect { unsafe { ffi :: CGRectApplyAffineTransform (* self , * t) } } # [inline] pub fn contains (& self , point : & CGPoint) -> bool { unsafe { ffi :: CGRectContainsPoint (* self , * point) == 1 } } }
};
}
