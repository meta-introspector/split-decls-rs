// Generated macro for impl_204 (impl)
macro_rules! Depcrate_features_serde_serimpl_204 {
() => {
// Module: crate::features::serde::ser
// Provides: {"impl_204"}
// Dependencies: {}
impl < ENC : Encoder > SerializeTupleVariant for Compound < '_ , ENC > { type Ok = () ; type Error = EncodeError ; fn serialize_field < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : Serialize + ? Sized , { value . serialize (SerdeEncoder { enc : self . enc }) } fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (()) } }
};
}
