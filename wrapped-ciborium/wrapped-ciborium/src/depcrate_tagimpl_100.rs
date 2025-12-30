// Generated macro for impl_100 (impl)
macro_rules! Depcrate_tagimpl_100 {
() => {
// Module: crate::tag
// Provides: {"impl_100"}
// Dependencies: {}
impl ser :: SerializeTupleStruct for Serializer { type Ok = u64 ; type Error = Error ; # [inline] fn serialize_field < U : ? Sized + ser :: Serialize > (& mut self , _value : & U) -> Result < () , Error > { Err (Error) } # [inline] fn end (self) -> Result < Self :: Ok , Self :: Error > { Err (Error) } }
};
}
