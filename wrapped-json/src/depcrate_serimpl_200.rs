// Generated macro for impl_200 (impl)
macro_rules! Depcrate_serimpl_200 {
() => {
// Module: crate::ser
// Provides: {"impl_200"}
// Dependencies: {}
impl < 'a , W , F > ser :: SerializeSeq for Compound < 'a , W , F > where W : io :: Write , F : Formatter , { type Ok = () ; type Error = Error ; # [inline] fn serialize_element < T > (& mut self , value : & T) -> Result < () > where T : ? Sized + Serialize , { match self { Compound :: Map { ser , state } => { tri ! (ser . formatter . begin_array_value (& mut ser . writer , * state == State :: First) . map_err (Error :: io)) ; * state = State :: Rest ; tri ! (value . serialize (& mut ** ser)) ; ser . formatter . end_array_value (& mut ser . writer) . map_err (Error :: io) } # [cfg (feature = "arbitrary_precision")] Compound :: Number { .. } => unreachable ! () , # [cfg (feature = "raw_value")] Compound :: RawValue { .. } => unreachable ! () , } } # [inline] fn end (self) -> Result < () > { match self { Compound :: Map { ser , state } => match state { State :: Empty => Ok (()) , _ => ser . formatter . end_array (& mut ser . writer) . map_err (Error :: io) , } , # [cfg (feature = "arbitrary_precision")] Compound :: Number { .. } => unreachable ! () , # [cfg (feature = "raw_value")] Compound :: RawValue { .. } => unreachable ! () , } } }
};
}
