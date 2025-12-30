// Generated macro for impl_12 (impl)
macro_rules! Depcrate_errorimpl_12 {
() => {
// Module: crate::error
// Provides: {"impl_12"}
// Dependencies: {}
impl From < VerboseError < & str > > for VerboseError < String > { fn from (value : VerboseError < & str >) -> Self { VerboseError { errors : value . errors . into_iter () . map (| (i , e) | (i . to_owned () , e)) . collect () , } } }
};
}
