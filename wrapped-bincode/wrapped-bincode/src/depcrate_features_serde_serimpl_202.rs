// Generated macro for impl_202 (impl)
macro_rules! Depcrate_features_serde_serimpl_202 {
() => {
// Module: crate::features::serde::ser
// Provides: {"impl_202"}
// Dependencies: {}
impl < ENC : Encoder > SerializeTuple for Compound < '_ , ENC > { type Ok = () ; type Error = EncodeError ; fn serialize_element < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : Serialize + ? Sized , { value . serialize (SerdeEncoder { enc : self . enc }) } fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (()) } }
};
}
