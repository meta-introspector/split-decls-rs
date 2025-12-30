// Generated macro for mockable_item_impl (function)
macro_rules! Depcrate_mockable_structmockable_item_impl {
() => {
// Module: crate::mockable_struct
// Provides: {"mockable_item_impl"}
// Dependencies: {}
# [doc = " Performs transformations on the ItemImpl to make it mockable"] fn mockable_item_impl (mut impl_ : ItemImpl , name : & Ident , generics : & Generics) -> ItemImpl { mock_ident_in_type (& mut impl_ . self_ty) ; for item in impl_ . items . iter_mut () { if let ImplItem :: Fn (ref mut iim) = item { mockable_method (iim , name , generics) ; } } impl_ }
};
}
