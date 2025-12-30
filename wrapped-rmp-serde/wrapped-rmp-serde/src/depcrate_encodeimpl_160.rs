// Generated macro for impl_160 (impl)
macro_rules! Depcrate_encodeimpl_160 {
() => {
// Module: crate::encode
// Provides: {"impl_160"}
// Dependencies: {}
impl < 'a , W : Write + 'a , C : SerializerConfig > SerializeTuple for Compound < 'a , W , C > { type Ok = () ; type Error = Error ; # [inline] fn serialize_element < T : ? Sized + Serialize > (& mut self , value : & T) -> Result < () , Self :: Error > { value . serialize (& mut * self . se) } # [inline (always)] fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (()) } }
};
}
