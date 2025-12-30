// Generated macro for impl_177 (impl)
macro_rules! Depcrate_value_serimpl_177 {
() => {
// Module: crate::value::ser
// Provides: {"impl_177"}
// Dependencies: {}
impl ser :: SerializeSeq for Serializer < Vec < Value > > { type Ok = Value ; type Error = Error ; # [inline] fn serialize_element < U : ? Sized + ser :: Serialize > (& mut self , value : & U) -> Result < () , Error > { self . 0 . push (Value :: serialized (& value) ?) ; Ok (()) } # [inline] fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (self . 0 . into ()) } }
};
}
