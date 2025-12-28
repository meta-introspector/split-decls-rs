macro_rules! deps {
    () => {
        Formatter!();
        RawValue!();
        RawValueStrEmitter!();
        Error!();
        Number!();
        NumberStrEmitter!();
        Result!();
        Map!();
        SerializeMap!();
        Compound!();
    };
}

macro_rules! impl_201 {
    () => {
        deps!();
        impl < 'a , W , F > ser :: SerializeStruct for Compound < 'a , W , F > where W : io :: Write , F : Formatter , { type Ok = () ; type Error = Error ; # [inline] fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () > where T : ? Sized + Serialize , { match self { Compound :: Map { .. } => ser :: SerializeMap :: serialize_entry (self , key , value) , # [cfg (feature = "arbitrary_precision")] Compound :: Number { ser , .. } => { if key == crate :: number :: TOKEN { value . serialize (NumberStrEmitter (ser)) } else { Err (invalid_number ()) } } # [cfg (feature = "raw_value")] Compound :: RawValue { ser , .. } => { if key == crate :: raw :: TOKEN { value . serialize (RawValueStrEmitter (ser)) } else { Err (invalid_raw_value ()) } } } } # [inline] fn end (self) -> Result < () > { match self { Compound :: Map { .. } => ser :: SerializeMap :: end (self) , # [cfg (feature = "arbitrary_precision")] Compound :: Number { .. } => Ok (()) , # [cfg (feature = "raw_value")] Compound :: RawValue { .. } => Ok (()) , } } }
    };
}

impl_201!()