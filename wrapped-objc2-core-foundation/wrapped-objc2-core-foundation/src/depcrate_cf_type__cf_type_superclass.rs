// Generated macro for __cf_type_superclass (macro)
macro_rules! Depcrate_cf_type__cf_type_superclass {
() => {
// Module: crate::cf_type
// Provides: {"__cf_type_superclass"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __cf_type_superclass { (impl ($ ($ generics : tt) *) $ ty : ty) => { impl $ ($ generics) * $ crate :: __cf_macro_helpers :: Deref for $ ty { type Target = $ crate :: CFType ; # [inline] fn deref (& self) -> & Self :: Target { unsafe { $ crate :: __cf_macro_helpers :: transmute (self) } } } } ; (impl ($ ($ generics : tt) *) $ ty : ty : $ superclass : ty) => { impl $ ($ generics) * $ crate :: __cf_macro_helpers :: Deref for $ ty { type Target = $ superclass ; # [inline] fn deref (& self) -> & Self :: Target { unsafe { $ crate :: __cf_macro_helpers :: transmute (self) } } } impl $ ($ generics) * $ crate :: __cf_macro_helpers :: AsRef <$ superclass > for $ ty { # [inline] fn as_ref (& self) -> &$ superclass { self } } impl $ ($ generics) * $ crate :: __cf_macro_helpers :: Borrow <$ superclass > for $ ty { # [inline] fn borrow (& self) -> &$ superclass { self } } } ; }
};
}
