// Generated macro for impl_164 (impl)
macro_rules! Depcrate_dataimpl_164 {
() => {
// Module: crate::data
// Provides: {"impl_164"}
// Dependencies: {}
impl Extend < u8 > for & NSMutableData { # [doc = " You should use [`extend_from_slice`] whenever possible, it is more"] # [doc = " performant."] # [doc = ""] # [doc = " [`extend_from_slice`]: NSMutableData::extend_from_slice"] fn extend < T : IntoIterator < Item = u8 > > (& mut self , iter : T) { let iterator = iter . into_iter () ; iterator . for_each (move | item | self . push (item)) ; } }
};
}
