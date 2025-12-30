// Generated macro for impl_167 (impl)
macro_rules! Depcrate_serializerimpl_167 {
() => {
// Module: crate::serializer
// Provides: {"impl_167"}
// Dependencies: {}
impl < 'a , 'w , W : io :: Write > SerializeTupleVariant for & 'a mut SeHeader < 'w , W > { type Ok = () ; type Error = Error ; fn serialize_field < T : ? Sized + Serialize > (& mut self , _value : & T ,) -> Result < () , Self :: Error > { unreachable ! () } fn end (self) -> Result < Self :: Ok , Self :: Error > { unreachable ! () } }
};
}
