// Generated macro for Error (enum)
macro_rules! Depcrate_multi_dirichletError {
() => {
// Module: crate::multi::dirichlet
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Error type returned from [`Dirichlet::new`]."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Error { # [doc = " `alpha.len() < 2`."] AlphaTooShort , # [doc = " `alpha <= 0.0` or `nan`."] AlphaTooSmall , # [doc = " `alpha` is subnormal."] # [doc = " Variate generation methods are not reliable with subnormal inputs."] AlphaSubnormal , # [doc = " `alpha` is infinite."] AlphaInfinite , # [doc = " Failed to create required Gamma distribution(s)."] FailedToCreateGamma , # [doc = " Failed to create required Beta distribition(s)."] FailedToCreateBeta , # [doc = " `size < 2`."] SizeTooSmall , }
};
}
