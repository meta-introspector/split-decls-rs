// Generated macro for RecoveryId (struct)
macro_rules! Depcrate_recoveryRecoveryId {
() => {
// Module: crate::recovery
// Provides: {"RecoveryId"}
// Dependencies: {}
# [doc = " Recovery IDs, a.k.a. \"recid\"."] # [doc = ""] # [doc = " This is an integer value `0`, `1`, `2`, or `3` included along with a"] # [doc = " signature which is used during the recovery process to select the correct"] # [doc = " public key from the signature."] # [doc = ""] # [doc = " It consists of two bits of information:"] # [doc = ""] # [doc = " - low bit (0/1): was the y-coordinate of the affine point resulting from"] # [doc = "   the fixed-base multiplication 𝑘×𝑮 odd? This part of the algorithm"] # [doc = "   functions similar to point decompression."] # [doc = " - hi bit (2/3): did the affine x-coordinate of 𝑘×𝑮 overflow the order of"] # [doc = "   the scalar field, requiring a reduction when computing `r`?"] # [derive (Copy , Clone , Debug , Eq , PartialEq , PartialOrd , Ord)] pub struct RecoveryId (pub (crate) u8) ;
};
}
