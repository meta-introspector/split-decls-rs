// Generated macro for impl_94 (impl)
macro_rules! Depcrate_arrayimpl_94 {
() => {
// Module: crate::array
// Provides: {"impl_94"}
// Dependencies: {}
impl < 'a , ObjectType : Message > Extend < & 'a ObjectType > for & NSMutableArray < ObjectType > { fn extend < I : IntoIterator < Item = & 'a ObjectType > > (& mut self , iter : I) { iter . into_iter () . for_each (move | item | self . addObject (item)) ; } }
};
}
