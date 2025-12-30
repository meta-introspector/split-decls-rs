// Generated macro for impl_180 (impl)
macro_rules! Depcrate_value_serimpl_180 {
() => {
// Module: crate::value::ser
// Provides: {"impl_180"}
// Dependencies: {}
impl ser :: SerializeTupleVariant for Serializer < Named < Vec < Value > > > { type Ok = Value ; type Error = Error ; # [inline] fn serialize_field < U : ? Sized + ser :: Serialize > (& mut self , value : & U) -> Result < () , Error > { match self . 0 . tag . as_mut () { Some (tag) => match tag . tag { None => match value . serialize (crate :: tag :: Serializer) { Ok (t) => tag . tag = Some (t) , Err (..) => return Err (ser :: Error :: custom ("expected tag")) , } , Some (..) => tag . val = Some (Value :: serialized (value) ?) , } , None => self . 0 . data . push (Value :: serialized (value) ?) , } Ok (()) } # [inline] fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (match self . 0 . tag { Some (tag) => match tag { Tagged { tag : Some (t) , val : Some (v) , } => Value :: Tag (t , v . into ()) , _ => return Err (ser :: Error :: custom ("invalid tag input")) , } , None => vec ! [(self . 0 . name . into () , self . 0 . data . into ())] . into () , }) } }
};
}
