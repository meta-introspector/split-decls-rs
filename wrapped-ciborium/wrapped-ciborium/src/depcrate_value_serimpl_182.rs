// Generated macro for impl_182 (impl)
macro_rules! Depcrate_value_serimpl_182 {
() => {
// Module: crate::value::ser
// Provides: {"impl_182"}
// Dependencies: {}
impl ser :: SerializeStruct for Serializer < Vec < (Value , Value) > > { type Ok = Value ; type Error = Error ; # [inline] fn serialize_field < U : ? Sized + ser :: Serialize > (& mut self , key : & 'static str , value : & U ,) -> Result < () , Error > { let k = Value :: serialized (& key) ? ; let v = Value :: serialized (& value) ? ; self . 0 . push ((k , v)) ; Ok (()) } # [inline] fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (self . 0 . into ()) } }
};
}
