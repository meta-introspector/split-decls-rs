// Generated macro for impl_149 (impl)
macro_rules! Depcrate_serializerimpl_149 {
() => {
// Module: crate::serializer
// Provides: {"impl_149"}
// Dependencies: {}
impl < 'a , 'w , W : io :: Write > SerializeSeq for & 'a mut SeRecord < 'w , W > { type Ok = () ; type Error = Error ; fn serialize_element < T : ? Sized + Serialize > (& mut self , value : & T ,) -> Result < () , Self :: Error > { value . serialize (& mut * * self) } fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (()) } }
};
}
