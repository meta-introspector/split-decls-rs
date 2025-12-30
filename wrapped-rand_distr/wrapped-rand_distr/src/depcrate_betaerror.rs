// Generated macro for Error (enum)
macro_rules! Depcrate_betaError {
() => {
// Module: crate::beta
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Error type returned from [`Beta::new`]."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub enum Error { # [doc = " `alpha <= 0` or `nan`."] AlphaTooSmall , # [doc = " `beta <= 0` or `nan`."] BetaTooSmall , }
};
}
