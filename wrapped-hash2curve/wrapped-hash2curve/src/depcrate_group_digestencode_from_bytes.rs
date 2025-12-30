// Generated macro for encode_from_bytes (function)
macro_rules! Depcrate_group_digestencode_from_bytes {
() => {
// Module: crate::group_digest
// Provides: {"encode_from_bytes"}
// Dependencies: {}
# [doc = " Computes the encode to curve routine."] # [doc = " See [`GroupDigest::encode_from_bytes()`] for more details."] # [doc = ""] # [doc = " For the `expand_message` call, `len_in_bytes = <Self::FieldElement as FromOkm>::Length`."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " When the chosen [`ExpandMsg`] implementation returns an error. See [`ExpandMsgXmdError`]"] # [doc = " and [`ExpandMsgXofError`] for examples."] # [doc = ""] # [doc = " [`ExpandMsgXmdError`]: crate::ExpandMsgXmdError"] # [doc = " [`ExpandMsgXofError`]: crate::ExpandMsgXofError"] pub fn encode_from_bytes < C , X > (msg : & [& [u8]] , dst : & [& [u8]]) -> Result < ProjectivePoint < C > , X :: Error > where C : MapToCurve , X : ExpandMsg < C :: SecurityLevel > , { let [u] = hash_to_field :: < 1 , X , _ , C :: FieldElement , C :: Length > (msg , dst) ? ; let q0 = C :: map_to_curve (u) ; Ok (q0 . clear_cofactor ()) }
};
}
