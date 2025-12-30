// Generated macro for impl_22 (impl)
macro_rules! Depcrate_iterimpl_22 {
() => {
// Module: crate::iter
// Provides: {"impl_22"}
// Dependencies: {}
impl < 'a , K : Eq + Hash + 'a , V : 'a > Iterator for Iter < 'a , K , V > { type Item = RefMulti < 'a , K , V > ; fn next (& mut self) -> Option < Self :: Item > { loop { if let Some (current) = self . current . as_mut () { if let Some ((k , v)) = current . 1 . next () { let guard = current . 0 . clone () ; return Some (RefMulti :: new (guard , k , v)) ; } } let guard = self . shards . next () ? . read () ; let (guard , shard) = unsafe { RwLockReadGuardDetached :: detach_from (guard) } ; let iter = shard . iter () ; self . current = Some ((Arc :: new (guard) , iter)) ; } } }
};
}
