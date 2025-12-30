// Generated macro for _ (const)
macro_rules! Depcrate_ {
() => {
// Module: crate
// Provides: {"_"}
// Dependencies: {}
const _ : () = { fn into_iter < T : Collect > () -> Iter < T > { let head = T :: registry () . head . load (Ordering :: Acquire) ; Iter { node : unsafe { head . as_ref () } , marker : PhantomData , } } impl < T : Collect > IntoIterator for iter < T > { type Item = & 'static T ; type IntoIter = Iter < T > ; fn into_iter (self) -> Self :: IntoIter { into_iter () } } # [doc (hidden)] impl < T : Collect > Deref for iter < T > { type Target = fn () -> Iter < T > ; fn deref (& self) -> & Self :: Target { & (into_iter as fn () -> Iter < T >) } } pub struct Iter < T : 'static > { node : Option < & 'static Node > , marker : PhantomData < T > , } impl < T : 'static > Iterator for Iter < T > { type Item = & 'static T ; fn next (& mut self) -> Option < Self :: Item > { let node = self . node ? ; unsafe { let value_ptr = (node . value as * const dyn ErasedNode) . cast :: < T > () ; self . node = * node . next . get () ; Some (& * value_ptr) } } } impl < T > Clone for Iter < T > { fn clone (& self) -> Self { Self { node : self . node , marker : PhantomData , } } } } ;
};
}
