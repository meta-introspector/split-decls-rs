// Generated macro for impl_166 (impl)
macro_rules! Depcrate_serializerimpl_166 {
() => {
// Module: crate::serializer
// Provides: {"impl_166"}
// Dependencies: {}
impl < 'a , 'w , W : io :: Write > SerializeTupleStruct for & 'a mut SeHeader < 'w , W > { type Ok = () ; type Error = Error ; fn serialize_field < T : ? Sized + Serialize > (& mut self , value : & T ,) -> Result < () , Self :: Error > { value . serialize (& mut * * self) } fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (()) } }
};
}
