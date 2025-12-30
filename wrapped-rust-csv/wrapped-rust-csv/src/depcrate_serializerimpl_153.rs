// Generated macro for impl_153 (impl)
macro_rules! Depcrate_serializerimpl_153 {
() => {
// Module: crate::serializer
// Provides: {"impl_153"}
// Dependencies: {}
impl < 'a , 'w , W : io :: Write > SerializeMap for & 'a mut SeRecord < 'w , W > { type Ok = () ; type Error = Error ; fn serialize_key < T : ? Sized + Serialize > (& mut self , _key : & T ,) -> Result < () , Self :: Error > { unreachable ! () } fn serialize_value < T : ? Sized + Serialize > (& mut self , _value : & T ,) -> Result < () , Self :: Error > { unreachable ! () } fn end (self) -> Result < Self :: Ok , Self :: Error > { unreachable ! () } }
};
}
