// Generated macro for impl_161 (impl)
macro_rules! Depcrate_encodeimpl_161 {
() => {
// Module: crate::encode
// Provides: {"impl_161"}
// Dependencies: {}
impl < 'a , W : Write + 'a , C : SerializerConfig > SerializeTupleStruct for Compound < 'a , W , C > { type Ok = () ; type Error = Error ; # [inline] fn serialize_field < T : ? Sized + Serialize > (& mut self , value : & T) -> Result < () , Self :: Error > { value . serialize (& mut * self . se) } # [inline (always)] fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (()) } }
};
}
