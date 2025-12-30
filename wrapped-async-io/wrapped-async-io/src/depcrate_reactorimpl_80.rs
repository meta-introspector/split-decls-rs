// Generated macro for impl_80 (impl)
macro_rules! Depcrate_reactorimpl_80 {
() => {
// Module: crate::reactor
// Provides: {"impl_80"}
// Dependencies: {}
impl < H : Borrow < crate :: Async < T > > , T > Drop for Ready < H , T > { fn drop (& mut self) { if let Some (key) = self . index { let mut state = self . handle . borrow () . source . state . lock () . unwrap () ; let wakers = & mut state [self . dir] . wakers ; if wakers . contains (key) { wakers . remove (key) ; } } } }
};
}
