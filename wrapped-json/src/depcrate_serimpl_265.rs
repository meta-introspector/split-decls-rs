// Generated macro for impl_265 (impl)
macro_rules! Depcrate_serimpl_265 {
() => {
// Module: crate::ser
// Provides: {"impl_265"}
// Dependencies: {}
impl < 'a , W , F > ser :: SerializeTupleVariant for Compound < 'a , W , F > where W : io :: Write , F : Formatter , { type Ok = () ; type Error = Error ; # [inline] fn serialize_field < T > (& mut self , value : & T) -> Result < () > where T : ? Sized + Serialize , { ser :: SerializeSeq :: serialize_element (self , value) } # [inline] fn end (self) -> Result < () > { match self { Compound :: Map { ser , state } => { match state { State :: Empty => { } _ => tri ! (ser . formatter . end_array (& mut ser . writer) . map_err (Error :: io)) , } tri ! (ser . formatter . end_object_value (& mut ser . writer) . map_err (Error :: io)) ; ser . formatter . end_object (& mut ser . writer) . map_err (Error :: io) } # [cfg (feature = "arbitrary_precision")] Compound :: Number { .. } => unreachable ! () , # [cfg (feature = "raw_value")] Compound :: RawValue { .. } => unreachable ! () , } } }
};
}
