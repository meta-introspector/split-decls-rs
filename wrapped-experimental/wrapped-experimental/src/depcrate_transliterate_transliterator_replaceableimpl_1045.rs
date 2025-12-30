// Generated macro for impl_1045 (impl)
macro_rules! Depcrate_transliterate_transliterator_replaceableimpl_1045 {
() => {
// Module: crate::transliterate::transliterator::replaceable
// Provides: {"impl_1045"}
// Dependencies: {}
impl < F > InsertableToReplaceableAdapter < '_ , '_ , F > where F : FnMut (usize) , { # [doc = " Returns a type that allows getting a `Replaceable` from it. The replaceable will"] # [doc = " transliterate everything since `self` was created with [`Insertable::start_replaceable_adapter`]."] pub (super) fn as_replaceable (& mut self) -> InsertableGuard < '_ , impl FnMut (& [u8]) + '_ > { self . child . make_contiguous () ; let range_end = self . child . curr ; let visible_range = self . range_start .. range_end ; let child = self . child . deref_mut () ; let hidden_len = child . _rep . content . len () - visible_range . len () ; let content = & mut child . _rep . content ; let modifiable_content = content . tighten (visible_range) ; let rep = unsafe { Replaceable :: from_hide (modifiable_content) } ; let child_curr = & mut child . curr ; let child_end_len = & child . end_len ; let on_drop = move | new_content : & [u8] | { * child_curr = new_content . len () + hidden_len - child_end_len ; } ; InsertableGuard :: new (rep , on_drop) } }
};
}
