// Generated macro for impl_179 (impl)
macro_rules! Depcrate_value_serimpl_179 {
() => {
// Module: crate::value::ser
// Provides: {"impl_179"}
// Dependencies: {}
impl ser :: SerializeTupleStruct for Serializer < Vec < Value > > { type Ok = Value ; type Error = Error ; # [inline] fn serialize_field < U : ? Sized + ser :: Serialize > (& mut self , value : & U) -> Result < () , Error > { self . 0 . push (Value :: serialized (& value) ?) ; Ok (()) } # [inline] fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (self . 0 . into ()) } }
};
}
