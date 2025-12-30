// Generated macro for FeeBin (struct)
macro_rules! DepcrateFeeBin {
() => {
// Module: crate
// Provides: {"FeeBin"}
// Dependencies: {}
# [doc = " A fee and its associated compute unit limit"] # [derive (Debug , Default , Clone , Eq , PartialEq)] pub struct FeeBin { # [doc = " maximum compute units for which this fee will be charged"] pub limit : u64 , # [doc = " fee in lamports"] pub fee : u64 , }
};
}
