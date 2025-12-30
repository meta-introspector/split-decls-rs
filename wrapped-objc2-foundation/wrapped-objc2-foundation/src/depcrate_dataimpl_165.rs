// Generated macro for impl_165 (impl)
macro_rules! Depcrate_dataimpl_165 {
() => {
// Module: crate::data
// Provides: {"impl_165"}
// Dependencies: {}
impl < 'a > Extend < & 'a u8 > for & NSMutableData { fn extend < T : IntoIterator < Item = & 'a u8 > > (& mut self , iter : T) { let iterator = iter . into_iter () ; iterator . for_each (move | item | self . push (* item)) ; } }
};
}
