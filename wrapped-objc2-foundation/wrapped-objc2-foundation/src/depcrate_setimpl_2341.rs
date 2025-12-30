// Generated macro for impl_2341 (impl)
macro_rules! Depcrate_setimpl_2341 {
() => {
// Module: crate::set
// Provides: {"impl_2341"}
// Dependencies: {}
impl < 'a , ObjectType : Message > Extend < & 'a ObjectType > for & NSMutableSet < ObjectType > { fn extend < I : IntoIterator < Item = & 'a ObjectType > > (& mut self , iter : I) { iter . into_iter () . for_each (move | item | { self . addObject (item) ; }) ; } }
};
}
