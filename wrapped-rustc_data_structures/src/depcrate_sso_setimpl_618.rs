// Generated macro for impl_618 (impl)
macro_rules! Depcrate_sso_setimpl_618 {
() => {
// Module: crate::sso::set
// Provides: {"impl_618"}
// Dependencies: {}
impl < 'a , T > IntoIterator for & 'a SsoHashSet < T > { type IntoIter = std :: iter :: Map < < & 'a SsoHashMap < T , () > as IntoIterator > :: IntoIter , fn ((& 'a T , & 'a ())) -> & 'a T , > ; type Item = < Self :: IntoIter as Iterator > :: Item ; # [inline] fn into_iter (self) -> Self :: IntoIter { self . map . iter () . map (entry_to_key) } }
};
}
