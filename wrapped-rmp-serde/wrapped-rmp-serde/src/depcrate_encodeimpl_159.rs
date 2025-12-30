// Generated macro for impl_159 (impl)
macro_rules! Depcrate_encodeimpl_159 {
() => {
// Module: crate::encode
// Provides: {"impl_159"}
// Dependencies: {}
impl < 'a , W : Write + 'a , C : SerializerConfig > SerializeSeq for Compound < 'a , W , C > { type Ok = () ; type Error = Error ; # [inline] fn serialize_element < T : ? Sized + Serialize > (& mut self , value : & T) -> Result < () , Self :: Error > { value . serialize (& mut * self . se) } # [inline (always)] fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (()) } }
};
}
