// Generated macro for impl_11 (impl)
macro_rules! Depcrate_errorimpl_11 {
() => {
// Module: crate::error
// Provides: {"impl_11"}
// Dependencies: {}
impl From < VerboseError < & [u8] > > for VerboseError < Vec < u8 > > { fn from (value : VerboseError < & [u8] >) -> Self { VerboseError { errors : value . errors . into_iter () . map (| (i , e) | (i . to_owned () , e)) . collect () , } } }
};
}
