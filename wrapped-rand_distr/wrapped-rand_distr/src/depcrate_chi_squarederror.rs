// Generated macro for Error (enum)
macro_rules! Depcrate_chi_squaredError {
() => {
// Module: crate::chi_squared
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Error type returned from [`ChiSquared::new`] and [`StudentT::new`](crate::StudentT::new)."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub enum Error { # [doc = " `0.5 * k <= 0` or `nan`."] DoFTooSmall , }
};
}
