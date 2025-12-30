// Generated macro for Error (enum)
macro_rules! Depcrate_distr_uniformError {
() => {
// Module: crate::distr::uniform
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Error type returned from [`Uniform::new`] and `new_inclusive`."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Error { # [doc = " `low > high`, or equal in case of exclusive range."] EmptyRange , # [doc = " Input or range `high - low` is non-finite. Not relevant to integer types."] NonFinite , }
};
}
