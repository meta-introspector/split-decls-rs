// Generated macro for impl_165 (impl)
macro_rules! Depcrate_serializerimpl_165 {
() => {
// Module: crate::serializer
// Provides: {"impl_165"}
// Dependencies: {}
impl < 'a , 'w , W : io :: Write > SerializeTuple for & 'a mut SeHeader < 'w , W > { type Ok = () ; type Error = Error ; fn serialize_element < T : ? Sized + Serialize > (& mut self , value : & T ,) -> Result < () , Self :: Error > { value . serialize (& mut * * self) } fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (()) } }
};
}
