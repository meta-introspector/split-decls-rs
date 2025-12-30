// Generated macro for impl_594 (impl)
macro_rules! Depcrate_read_lookupimpl_594 {
() => {
// Module: crate::read::lookup
// Provides: {"impl_594"}
// Dependencies: {}
impl < R , Parser > From < R > for DebugLookup < R , Parser > where R : Reader , Parser : LookupParser < R > , { fn from (input_buffer : R) -> Self { DebugLookup { input_buffer , phantom : PhantomData , } } }
};
}
