// Generated macro for impl_170 (impl)
macro_rules! Depcrate_serializerimpl_170 {
() => {
// Module: crate::serializer
// Provides: {"impl_170"}
// Dependencies: {}
impl < 'a , 'w , W : io :: Write > SerializeStructVariant for & 'a mut SeHeader < 'w , W > { type Ok = () ; type Error = Error ; fn serialize_field < T : ? Sized + Serialize > (& mut self , _key : & 'static str , _value : & T ,) -> Result < () , Self :: Error > { unreachable ! () } fn end (self) -> Result < Self :: Ok , Self :: Error > { unreachable ! () } }
};
}
