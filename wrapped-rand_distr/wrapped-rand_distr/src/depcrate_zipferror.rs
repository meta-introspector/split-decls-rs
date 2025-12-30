// Generated macro for Error (enum)
macro_rules! Depcrate_zipfError {
() => {
// Module: crate::zipf
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Error type returned from [`Zipf::new`]."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Error { # [doc = " `s < 0` or `nan`."] STooSmall , # [doc = " `n < 1`."] NTooSmall , }
};
}
