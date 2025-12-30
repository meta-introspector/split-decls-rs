// Generated macro for impl_103 (impl)
macro_rules! Depcrate_tagimpl_103 {
() => {
// Module: crate::tag
// Provides: {"impl_103"}
// Dependencies: {}
impl ser :: SerializeStruct for Serializer { type Ok = u64 ; type Error = Error ; # [inline] fn serialize_field < U : ? Sized + ser :: Serialize > (& mut self , _key : & 'static str , _value : & U ,) -> Result < () , Error > { Err (Error) } # [inline] fn end (self) -> Result < Self :: Ok , Self :: Error > { Err (Error) } }
};
}
