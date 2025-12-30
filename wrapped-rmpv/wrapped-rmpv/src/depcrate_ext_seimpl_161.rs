// Generated macro for impl_161 (impl)
macro_rules! Depcrate_ext_seimpl_161 {
() => {
// Module: crate::ext::se
// Provides: {"impl_161"}
// Dependencies: {}
impl ser :: SerializeStructVariant for SerializeStructVariant { type Ok = Value ; type Error = Error ; # [inline] fn serialize_field < T : ? Sized > (& mut self , _key : & 'static str , value : & T) -> Result < () , Error > where T : Serialize { self . vec . push (to_value (value) ?) ; Ok (()) } # [inline] fn end (self) -> Result < Value , Error > { Ok (Value :: Array (vec ! [Value :: from (self . idx) , Value :: Array (self . vec) ,])) } }
};
}
