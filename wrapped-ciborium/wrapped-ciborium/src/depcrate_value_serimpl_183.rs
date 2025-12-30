// Generated macro for impl_183 (impl)
macro_rules! Depcrate_value_serimpl_183 {
() => {
// Module: crate::value::ser
// Provides: {"impl_183"}
// Dependencies: {}
impl ser :: SerializeStructVariant for Serializer < Named < Vec < (Value , Value) > > > { type Ok = Value ; type Error = Error ; # [inline] fn serialize_field < U : ? Sized + ser :: Serialize > (& mut self , key : & 'static str , value : & U ,) -> Result < () , Self :: Error > { let k = Value :: serialized (& key) ? ; let v = Value :: serialized (& value) ? ; self . 0 . data . push ((k , v)) ; Ok (()) } # [inline] fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (vec ! [(self . 0 . name . into () , self . 0 . data . into ())] . into ()) } }
};
}
