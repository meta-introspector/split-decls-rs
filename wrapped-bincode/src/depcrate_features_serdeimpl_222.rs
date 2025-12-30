// Generated macro for impl_222 (impl)
macro_rules! Depcrate_features_serdeimpl_222 {
() => {
// Module: crate::features::serde
// Provides: {"impl_222"}
// Dependencies: {}
impl < T > crate :: Encode for Compat < T > where T : serde :: Serialize , { fn encode < E : crate :: enc :: Encoder > (& self , encoder : & mut E ,) -> Result < () , crate :: error :: EncodeError > { let serializer = ser :: SerdeEncoder { enc : encoder } ; self . 0 . serialize (serializer) ? ; Ok (()) } }
};
}
