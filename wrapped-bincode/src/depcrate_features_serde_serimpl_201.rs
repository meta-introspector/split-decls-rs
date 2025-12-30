// Generated macro for impl_201 (impl)
macro_rules! Depcrate_features_serde_serimpl_201 {
() => {
// Module: crate::features::serde::ser
// Provides: {"impl_201"}
// Dependencies: {}
impl < ENC : Encoder > SerializeSeq for Compound < '_ , ENC > { type Ok = () ; type Error = EncodeError ; fn serialize_element < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : Serialize + ? Sized , { value . serialize (SerdeEncoder { enc : self . enc }) } fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (()) } }
};
}
