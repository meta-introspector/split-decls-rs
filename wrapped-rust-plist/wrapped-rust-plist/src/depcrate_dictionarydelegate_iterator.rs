// Generated macro for delegate_iterator (macro)
macro_rules! Depcrate_dictionarydelegate_iterator {
() => {
// Module: crate::dictionary
// Provides: {"delegate_iterator"}
// Dependencies: {}
macro_rules ! delegate_iterator { (($ name : ident $ ($ generics : tt) *) => $ item : ty) => { impl $ ($ generics) * Iterator for $ name $ ($ generics) * { type Item = $ item ; # [inline] fn next (& mut self) -> Option < Self :: Item > { self . iter . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } } impl $ ($ generics) * ExactSizeIterator for $ name $ ($ generics) * { # [inline] fn len (& self) -> usize { self . iter . len () } } } }
};
}
