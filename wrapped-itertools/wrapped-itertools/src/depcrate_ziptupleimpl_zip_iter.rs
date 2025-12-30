// Generated macro for impl_zip_iter (macro)
macro_rules! Depcrate_ziptupleimpl_zip_iter {
() => {
// Module: crate::ziptuple
// Provides: {"impl_zip_iter"}
// Dependencies: {}
macro_rules ! impl_zip_iter { ($ ($ B : ident) ,*) => (# [allow (non_snake_case)] impl <$ ($ B : IntoIterator) ,*> From < ($ ($ B ,) *) > for Zip < ($ ($ B :: IntoIter ,) *) > { fn from (t : ($ ($ B ,) *)) -> Self { let ($ ($ B ,) *) = t ; Zip { t : ($ ($ B . into_iter () ,) *) } } } # [allow (non_snake_case)] # [allow (unused_assignments)] impl <$ ($ B) ,*> Iterator for Zip < ($ ($ B ,) *) > where $ ($ B : Iterator ,) * { type Item = ($ ($ B :: Item ,) *) ; fn next (& mut self) -> Option < Self :: Item > { let ($ (ref mut $ B ,) *) = self . t ; $ (let $ B = match $ B . next () { None => return None , Some (elt) => elt } ;) * Some (($ ($ B ,) *)) } fn size_hint (& self) -> (usize , Option < usize >) { let sh = (usize :: MAX , None) ; let ($ (ref $ B ,) *) = self . t ; $ (let sh = size_hint :: min ($ B . size_hint () , sh) ;) * sh } } # [allow (non_snake_case)] impl <$ ($ B) ,*> ExactSizeIterator for Zip < ($ ($ B ,) *) > where $ ($ B : ExactSizeIterator ,) * { } # [allow (non_snake_case)] impl <$ ($ B) ,*> DoubleEndedIterator for Zip < ($ ($ B ,) *) > where $ ($ B : DoubleEndedIterator + ExactSizeIterator ,) * { # [inline] fn next_back (& mut self) -> Option < Self :: Item > { let ($ (ref mut $ B ,) *) = self . t ; let size = * [$ ($ B . len () ,) *] . iter () . min () . unwrap () ; $ (if $ B . len () != size { for _ in 0 ..$ B . len () - size { $ B . next_back () ; } }) * match ($ ($ B . next_back () ,) *) { ($ (Some ($ B) ,) *) => Some (($ ($ B ,) *)) , _ => None , } } }) ; }
};
}
