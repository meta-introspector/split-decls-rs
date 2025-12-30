// Generated macro for SliceWriteError (enum)
macro_rules! DepcrateSliceWriteError {
() => {
// Module: crate
// Provides: {"SliceWriteError"}
// Dependencies: {}
# [doc = " Errors that could be returned by `Write` on `&mut [u8]`."] # [derive (Debug , Copy , Clone , Eq , PartialEq)] # [cfg_attr (feature = "defmt" , derive (defmt :: Format))] # [non_exhaustive] pub enum SliceWriteError { # [doc = " The target slice was full and so could not receive any new data."] Full , }
};
}
