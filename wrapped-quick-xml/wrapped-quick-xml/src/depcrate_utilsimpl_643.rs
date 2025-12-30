// Generated macro for impl_643 (impl)
macro_rules! Depcrate_utilsimpl_643 {
() => {
// Module: crate::utils
// Provides: {"impl_643"}
// Dependencies: {}
# [cfg (feature = "serialize")] impl < 'de > Deserialize < 'de > for ByteBuf { fn deserialize < D > (d : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { struct ValueVisitor ; impl < 'de > Visitor < 'de > for ValueVisitor { type Value = ByteBuf ; fn expecting (& self , f : & mut Formatter) -> fmt :: Result { f . write_str ("byte data") } fn visit_bytes < E : Error > (self , v : & [u8]) -> Result < Self :: Value , E > { Ok (ByteBuf (v . to_vec ())) } fn visit_byte_buf < E : Error > (self , v : Vec < u8 >) -> Result < Self :: Value , E > { Ok (ByteBuf (v)) } } d . deserialize_byte_buf (ValueVisitor) } }
};
}
