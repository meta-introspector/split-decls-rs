macro_rules! deps {
    () => {
        SerializeStructVariant!();
        Number!();
        Formatter!();
        Compound!();
        Error!();
        Result!();
        State!();
        RawValue!();
        Map!();
    };
}

macro_rules! impl_202 {
    () => {
        deps!();
        impl < 'a , W , F > ser :: SerializeStructVariant for Compound < 'a , W , F > where W : io :: Write , F : Formatter , { type Ok = () ; type Error = Error ; # [inline] fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () > where T : ? Sized + Serialize , { match * self { Compound :: Map { .. } => ser :: SerializeStruct :: serialize_field (self , key , value) , # [cfg (feature = "arbitrary_precision")] Compound :: Number { .. } => unreachable ! () , # [cfg (feature = "raw_value")] Compound :: RawValue { .. } => unreachable ! () , } } # [inline] fn end (self) -> Result < () > { match self { Compound :: Map { ser , state } => { match state { State :: Empty => { } _ => tri ! (ser . formatter . end_object (& mut ser . writer) . map_err (Error :: io)) , } tri ! (ser . formatter . end_object_value (& mut ser . writer) . map_err (Error :: io)) ; ser . formatter . end_object (& mut ser . writer) . map_err (Error :: io) } # [cfg (feature = "arbitrary_precision")] Compound :: Number { .. } => unreachable ! () , # [cfg (feature = "raw_value")] Compound :: RawValue { .. } => unreachable ! () , } } }
    };
}

impl_202!()