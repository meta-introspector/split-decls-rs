// Generated macro for impl_158 (impl)
macro_rules! Depcrate_ext_seimpl_158 {
() => {
// Module: crate::ext::se
// Provides: {"impl_158"}
// Dependencies: {}
impl ser :: SerializeTupleVariant for SerializeTupleVariant { type Ok = Value ; type Error = Error ; # [inline] fn serialize_field < T : ? Sized > (& mut self , value : & T) -> Result < () , Error > where T : Serialize { self . vec . push (to_value (value) ?) ; Ok (()) } # [inline] fn end (self) -> Result < Value , Error > { Ok (Value :: Array (vec ! [Value :: from (self . idx) , Value :: Array (self . vec)])) } }
};
}
