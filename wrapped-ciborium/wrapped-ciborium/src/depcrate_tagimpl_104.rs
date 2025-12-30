// Generated macro for impl_104 (impl)
macro_rules! Depcrate_tagimpl_104 {
() => {
// Module: crate::tag
// Provides: {"impl_104"}
// Dependencies: {}
impl ser :: SerializeStructVariant for Serializer { type Ok = u64 ; type Error = Error ; # [inline] fn serialize_field < U : ? Sized + ser :: Serialize > (& mut self , _key : & 'static str , _value : & U ,) -> Result < () , Self :: Error > { Err (Error) } # [inline] fn end (self) -> Result < Self :: Ok , Self :: Error > { Err (Error) } }
};
}
