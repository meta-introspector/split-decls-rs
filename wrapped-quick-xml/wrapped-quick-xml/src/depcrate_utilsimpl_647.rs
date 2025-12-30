// Generated macro for impl_647 (impl)
macro_rules! Depcrate_utilsimpl_647 {
() => {
// Module: crate::utils
// Provides: {"impl_647"}
// Dependencies: {}
# [cfg (feature = "serialize")] impl < 'de > Deserialize < 'de > for Bytes < 'de > { fn deserialize < D > (d : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { struct ValueVisitor ; impl < 'de > Visitor < 'de > for ValueVisitor { type Value = Bytes < 'de > ; fn expecting (& self , f : & mut Formatter) -> fmt :: Result { f . write_str ("borrowed bytes") } fn visit_borrowed_bytes < E : Error > (self , v : & 'de [u8]) -> Result < Self :: Value , E > { Ok (Bytes (v)) } } d . deserialize_bytes (ValueVisitor) } }
};
}
