// Generated macro for impl_146 (impl)
macro_rules! Depcrate_astimpl_146 {
() => {
// Module: crate::ast
// Provides: {"impl_146"}
// Dependencies: {}
impl < 'a > BorrowedType < 'a > { # [doc = " Creates a [`NonNull`] [`BorrowedType`] literal from the provided `name`."] pub (crate) fn non_null (name : & 'a str) -> Self { Self { name , modifiers : & [TypeModifier :: NonNull] , } } # [doc = " Borrows the inner [`Type`] of this [`List`] [`Type`], if it represents one."] pub (crate) fn borrow_list_inner (& self) -> Option < Self > { let mut out = None ; for (n , m) in self . modifiers . iter () . enumerate () . rev () { match m { TypeModifier :: NonNull => { } TypeModifier :: List (..) => { out = Some (Self { name : self . name , modifiers : & self . modifiers [.. n] , }) ; break ; } } } out } }
};
}
