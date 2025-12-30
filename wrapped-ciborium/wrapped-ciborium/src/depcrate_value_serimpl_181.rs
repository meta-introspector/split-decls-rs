// Generated macro for impl_181 (impl)
macro_rules! Depcrate_value_serimpl_181 {
() => {
// Module: crate::value::ser
// Provides: {"impl_181"}
// Dependencies: {}
impl ser :: SerializeMap for Serializer < Map > { type Ok = Value ; type Error = Error ; # [inline] fn serialize_key < U : ? Sized + ser :: Serialize > (& mut self , key : & U) -> Result < () , Error > { self . 0 . temp = Some (Value :: serialized (key) ?) ; Ok (()) } # [inline] fn serialize_value < U : ? Sized + ser :: Serialize > (& mut self , value : & U) -> Result < () , Error > { let key = self . 0 . temp . take () . unwrap () ; let val = Value :: serialized (& value) ? ; self . 0 . data . push ((key , val)) ; Ok (()) } # [inline] fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (self . 0 . data . into ()) } }
};
}
