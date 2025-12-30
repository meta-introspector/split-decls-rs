// Generated macro for impl_154 (impl)
macro_rules! Depcrate_serializerimpl_154 {
() => {
// Module: crate::serializer
// Provides: {"impl_154"}
// Dependencies: {}
impl < 'a , 'w , W : io :: Write > SerializeStruct for & 'a mut SeRecord < 'w , W > { type Ok = () ; type Error = Error ; fn serialize_field < T : ? Sized + Serialize > (& mut self , _key : & 'static str , value : & T ,) -> Result < () , Self :: Error > { value . serialize (& mut * * self) } fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (()) } }
};
}
