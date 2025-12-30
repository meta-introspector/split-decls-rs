// Generated macro for impl_99 (impl)
macro_rules! Depcrate_tagimpl_99 {
() => {
// Module: crate::tag
// Provides: {"impl_99"}
// Dependencies: {}
impl ser :: SerializeTuple for Serializer { type Ok = u64 ; type Error = Error ; # [inline] fn serialize_element < U : ? Sized + ser :: Serialize > (& mut self , _value : & U) -> Result < () , Error > { Err (Error) } # [inline] fn end (self) -> Result < Self :: Ok , Self :: Error > { Err (Error) } }
};
}
