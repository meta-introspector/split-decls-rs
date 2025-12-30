// Generated macro for impl_227 (impl)
macro_rules! Depcrate_features_serdeimpl_227 {
() => {
// Module: crate::features::serde
// Provides: {"impl_227"}
// Dependencies: {}
impl < T > crate :: Encode for BorrowCompat < T > where T : serde :: Serialize , { fn encode < E : crate :: enc :: Encoder > (& self , encoder : & mut E ,) -> Result < () , crate :: error :: EncodeError > { let serializer = ser :: SerdeEncoder { enc : encoder } ; self . 0 . serialize (serializer) ? ; Ok (()) } }
};
}
