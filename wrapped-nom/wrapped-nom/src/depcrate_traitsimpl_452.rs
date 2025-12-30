// Generated macro for impl_452 (impl)
macro_rules! Depcrate_traitsimpl_452 {
() => {
// Module: crate::traits
// Provides: {"impl_452"}
// Dependencies: {}
impl < I > ErrorConvert < error :: Error < (I , usize) > > for error :: Error < I > { fn convert (self) -> error :: Error < (I , usize) > { error :: Error { input : (self . input , 0) , code : self . code , } } }
};
}
