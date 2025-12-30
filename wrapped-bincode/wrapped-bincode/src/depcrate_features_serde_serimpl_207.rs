// Generated macro for impl_207 (impl)
macro_rules! Depcrate_features_serde_serimpl_207 {
() => {
// Module: crate::features::serde::ser
// Provides: {"impl_207"}
// Dependencies: {}
impl < ENC : Encoder > SerializeStructVariant for Compound < '_ , ENC > { type Ok = () ; type Error = EncodeError ; fn serialize_field < T > (& mut self , _key : & 'static str , value : & T) -> Result < () , Self :: Error > where T : Serialize + ? Sized , { value . serialize (SerdeEncoder { enc : self . enc }) } fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (()) } }
};
}
