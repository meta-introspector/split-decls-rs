// Generated macro for hash_to_scalar (function)
macro_rules! Depcrate_group_digesthash_to_scalar {
() => {
// Module: crate::group_digest
// Provides: {"hash_to_scalar"}
// Dependencies: {}
# [doc = " Computes the hash to field routine according to"] # [doc = " <https://www.rfc-editor.org/rfc/rfc9380.html#section-5-4>"] # [doc = " and returns a scalar."] # [doc = "   "] # [doc = " For the `expand_message` call, `len_in_bytes = <Self::FieldElement as FromOkm>::Length`."] # [doc = " This value must be less than `u16::MAX` or otherwise a compiler error will occur."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " When the chosen [`ExpandMsg`] implementation returns an error. See [`ExpandMsgXmdError`]"] # [doc = " and [`ExpandMsgXofError`] for examples."] # [doc = ""] # [doc = " [`ExpandMsgXmdError`]: crate::ExpandMsgXmdError"] # [doc = " [`ExpandMsgXofError`]: crate::ExpandMsgXofError"] pub fn hash_to_scalar < C , X , L > (msg : & [& [u8]] , dst : & [& [u8]]) -> Result < C :: Scalar , X :: Error > where C : MapToCurve , X : ExpandMsg < C :: SecurityLevel > , L : ArraySize + NonZero , C :: Scalar : Reduce < Array < u8 , L > > , { let [u] = hash_to_field :: < 1 , X , _ , C :: Scalar , L > (msg , dst) ? ; Ok (u) }
};
}
