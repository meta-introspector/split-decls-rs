// Generated macro for impl_451 (impl)
macro_rules! Depcrate_traitsimpl_451 {
() => {
// Module: crate::traits
// Provides: {"impl_451"}
// Dependencies: {}
impl < I > ErrorConvert < error :: Error < I > > for error :: Error < (I , usize) > { fn convert (self) -> error :: Error < I > { error :: Error { input : self . input . 0 , code : self . code , } } }
};
}
