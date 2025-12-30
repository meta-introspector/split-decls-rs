// Generated macro for impl_CFTypeDescription (macro)
macro_rules! Depcrateimpl_CFTypeDescription {
() => {
// Module: crate
// Provides: {"impl_CFTypeDescription"}
// Dependencies: {}
# [doc = " Implement `std::fmt::Debug` for the given type."] # [doc = ""] # [doc = " This will invoke the implementation of `Debug` for [`CFType`]"] # [doc = " which invokes [`CFCopyDescription`]."] # [doc = ""] # [doc = " The type must have an implementation of the [`TCFType`] trait, usually"] # [doc = " provided using the [`impl_TCFType`] macro."] # [doc = ""] # [doc = " [`CFType`]: base/struct.CFType.html#impl-Debug"] # [doc = " [`CFCopyDescription`]: https://developer.apple.com/documentation/corefoundation/1521252-cfcopydescription?language=objc"] # [doc = " [`TCFType`]: base/trait.TCFType.html"] # [doc = " [`impl_TCFType`]: macro.impl_TCFType.html"] # [macro_export] macro_rules ! impl_CFTypeDescription { ($ ty : ident) => { impl_CFTypeDescription ! ($ ty <>) ; } ; ($ ty : ident <$ ($ p : ident $ (: $ bound : path) *) ,*>) => { # [allow (unused_imports)] impl <$ ($ p $ (: $ bound) *) ,*> :: std :: fmt :: Debug for $ ty <$ ($ p) ,*> { fn fmt (& self , f : & mut :: std :: fmt :: Formatter) -> :: std :: fmt :: Result { use $ crate :: base :: TCFType ; self . as_CFType () . fmt (f) } } } }
};
}
