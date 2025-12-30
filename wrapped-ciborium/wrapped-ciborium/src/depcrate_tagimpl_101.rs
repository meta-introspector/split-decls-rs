// Generated macro for impl_101 (impl)
macro_rules! Depcrate_tagimpl_101 {
() => {
// Module: crate::tag
// Provides: {"impl_101"}
// Dependencies: {}
impl ser :: SerializeTupleVariant for Serializer { type Ok = u64 ; type Error = Error ; # [inline] fn serialize_field < U : ? Sized + ser :: Serialize > (& mut self , _value : & U) -> Result < () , Error > { Err (Error) } # [inline] fn end (self) -> Result < Self :: Ok , Self :: Error > { Err (Error) } }
};
}
