// Generated macro for impl_93 (impl)
macro_rules! Depcrate_arrayimpl_93 {
() => {
// Module: crate::array
// Provides: {"impl_93"}
// Dependencies: {}
impl < ObjectType : Message > Extend < Retained < ObjectType > > for & NSMutableArray < ObjectType > { fn extend < I : IntoIterator < Item = Retained < ObjectType > > > (& mut self , iter : I) { iter . into_iter () . for_each (move | item | self . addObject (& item)) ; } }
};
}
