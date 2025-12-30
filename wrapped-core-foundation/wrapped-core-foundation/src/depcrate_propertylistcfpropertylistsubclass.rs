// Generated macro for CFPropertyListSubClass (trait)
macro_rules! Depcrate_propertylistCFPropertyListSubClass {
() => {
// Module: crate::propertylist
// Provides: {"CFPropertyListSubClass"}
// Dependencies: {}
# [doc = " Trait for all subclasses of [`CFPropertyList`]."] # [doc = ""] # [doc = " [`CFPropertyList`]: struct.CFPropertyList.html"] pub trait CFPropertyListSubClass : TCFType { # [doc = " Create an instance of the superclass type [`CFPropertyList`] for this instance."] # [doc = ""] # [doc = " [`CFPropertyList`]: struct.CFPropertyList.html"] # [inline] fn to_CFPropertyList (& self) -> CFPropertyList { unsafe { CFPropertyList :: wrap_under_get_rule (self . as_concrete_TypeRef () . as_void_ptr ()) } } # [doc = " Equal to [`to_CFPropertyList`], but consumes self and avoids changing the reference count."] # [doc = ""] # [doc = " [`to_CFPropertyList`]: #method.to_CFPropertyList"] # [inline] fn into_CFPropertyList (self) -> CFPropertyList where Self : Sized , { let reference = self . as_concrete_TypeRef () . as_void_ptr () ; mem :: forget (self) ; unsafe { CFPropertyList :: wrap_under_create_rule (reference) } } }
};
}
