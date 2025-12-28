macro_rules! deps {
    () => {
        Result!();
        Compound!();
        Error!();
        Formatter!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl < 'a , W , F > ser :: SerializeTupleStruct for Compound < 'a , W , F > where W : io :: Write , F : Formatter , { type Ok = () ; type Error = Error ; # [inline] fn serialize_field < T > (& mut self , value : & T) -> Result < () > where T : ? Sized + Serialize , { ser :: SerializeSeq :: serialize_element (self , value) } # [inline] fn end (self) -> Result < () > { ser :: SerializeSeq :: end (self) } }
    };
}

impl_146!()