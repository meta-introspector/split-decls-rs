// Generated macro for impl_2340 (impl)
macro_rules! Depcrate_setimpl_2340 {
() => {
// Module: crate::set
// Provides: {"impl_2340"}
// Dependencies: {}
impl < ObjectType : Message > Extend < Retained < ObjectType > > for & NSMutableSet < ObjectType > { fn extend < I : IntoIterator < Item = Retained < ObjectType > > > (& mut self , iter : I) { iter . into_iter () . for_each (move | item | { self . addObject (& item) ; }) ; } }
};
}
