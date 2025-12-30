// Generated macro for impl_174 (impl)
macro_rules! Depcrate_encodeimpl_174 {
() => {
// Module: crate::encode
// Provides: {"impl_174"}
// Dependencies: {}
impl < 'a , W : Write + 'a > SerializeTuple for & mut ExtSerializer < 'a , W > { type Ok = () ; type Error = Error ; # [inline] fn serialize_element < T : ? Sized + Serialize > (& mut self , value : & T) -> Result < () , Self :: Error > { value . serialize (& mut self . fields_se) } # [inline (always)] fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (()) } }
};
}
