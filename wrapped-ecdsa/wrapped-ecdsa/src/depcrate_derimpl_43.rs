// Generated macro for impl_43 (impl)
macro_rules! Depcrate_derimpl_43 {
() => {
// Module: crate::der
// Provides: {"impl_43"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < C > Serialize for Signature < C > where C : EcdsaCurve , MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < MaxOverhead > + ArraySize , { fn serialize < S > (& self , serializer : S) -> core :: result :: Result < S :: Ok , S :: Error > where S : ser :: Serializer , { serdect :: slice :: serialize_hex_upper_or_bin (& self . as_bytes () , serializer) } }
};
}
