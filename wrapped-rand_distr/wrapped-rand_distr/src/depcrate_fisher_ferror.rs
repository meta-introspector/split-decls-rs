// Generated macro for Error (enum)
macro_rules! Depcrate_fisher_fError {
() => {
// Module: crate::fisher_f
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Error type returned from [`FisherF::new`]."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub enum Error { # [doc = " `m <= 0` or `nan`."] MTooSmall , # [doc = " `n <= 0` or `nan`."] NTooSmall , }
};
}
