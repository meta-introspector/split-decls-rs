// Generated macro for impl_13 (impl)
macro_rules! Depcrate_errorimpl_13 {
() => {
// Module: crate::error
// Provides: {"impl_13"}
// Dependencies: {}
impl < I > ErrorConvert < VerboseError < I > > for VerboseError < (I , usize) > { fn convert (self) -> VerboseError < I > { VerboseError { errors : self . errors . into_iter () . map (| (i , e) | (i . 0 , e)) . collect () , } } }
};
}
