// Generated macro for impl_14 (impl)
macro_rules! Depcrate_errorimpl_14 {
() => {
// Module: crate::error
// Provides: {"impl_14"}
// Dependencies: {}
impl < I > ErrorConvert < VerboseError < (I , usize) > > for VerboseError < I > { fn convert (self) -> VerboseError < (I , usize) > { VerboseError { errors : self . errors . into_iter () . map (| (i , e) | ((i , 0) , e)) . collect () , } } }
};
}
