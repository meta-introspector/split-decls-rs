// Generated macro for impl_203 (impl)
macro_rules! Depcrate_features_serde_serimpl_203 {
() => {
// Module: crate::features::serde::ser
// Provides: {"impl_203"}
// Dependencies: {}
impl < ENC : Encoder > SerializeTupleStruct for Compound < '_ , ENC > { type Ok = () ; type Error = EncodeError ; fn serialize_field < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : Serialize + ? Sized , { value . serialize (SerdeEncoder { enc : self . enc }) } fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (()) } }
};
}
