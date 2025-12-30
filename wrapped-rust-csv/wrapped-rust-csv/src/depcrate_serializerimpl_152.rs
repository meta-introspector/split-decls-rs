// Generated macro for impl_152 (impl)
macro_rules! Depcrate_serializerimpl_152 {
() => {
// Module: crate::serializer
// Provides: {"impl_152"}
// Dependencies: {}
impl < 'a , 'w , W : io :: Write > SerializeTupleVariant for & 'a mut SeRecord < 'w , W > { type Ok = () ; type Error = Error ; fn serialize_field < T : ? Sized + Serialize > (& mut self , _value : & T ,) -> Result < () , Self :: Error > { unreachable ! () } fn end (self) -> Result < Self :: Ok , Self :: Error > { unreachable ! () } }
};
}
