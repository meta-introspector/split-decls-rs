// Generated macro for hash_from_bytes (function)
macro_rules! Depcrate_group_digesthash_from_bytes {
() => {
// Module: crate::group_digest
// Provides: {"hash_from_bytes"}
// Dependencies: {}
# [doc = " Computes the hash to curve routine."] # [doc = " See [`GroupDigest::hash_from_bytes()`] for more details."] # [doc = ""] # [doc = " For the `expand_message` call, `len_in_bytes = <Self::FieldElement as FromOkm>::Length * 2`."] # [doc = " This value must be less than `u16::MAX` or otherwise a compiler error will occur."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " When the chosen [`ExpandMsg`] implementation returns an error. See [`ExpandMsgXmdError`]"] # [doc = " and [`ExpandMsgXofError`] for examples."] # [doc = ""] # [doc = " [`ExpandMsgXmdError`]: crate::ExpandMsgXmdError"] # [doc = " [`ExpandMsgXofError`]: crate::ExpandMsgXofError"] pub fn hash_from_bytes < C , X > (msg : & [& [u8]] , dst : & [& [u8]]) -> Result < ProjectivePoint < C > , X :: Error > where C : MapToCurve , X : ExpandMsg < C :: SecurityLevel > , { let [u0 , u1] = hash_to_field :: < 2 , X , _ , C :: FieldElement , C :: Length > (msg , dst) ? ; let q0 = C :: map_to_curve (u0) ; let q1 = C :: map_to_curve (u1) ; Ok ((q0 + q1) . clear_cofactor ()) }
};
}
