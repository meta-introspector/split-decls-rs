macro_rules! deps {
    () => {
        Error!();
        Result!();
        Formatter!();
        Compound!();
    };
}

macro_rules! impl_197 {
    () => {
        deps!();
        impl < 'a , W , F > ser :: SerializeTuple for Compound < 'a , W , F > where W : io :: Write , F : Formatter , { type Ok = () ; type Error = Error ; # [inline] fn serialize_element < T > (& mut self , value : & T) -> Result < () > where T : ? Sized + Serialize , { ser :: SerializeSeq :: serialize_element (self , value) } # [inline] fn end (self) -> Result < () > { ser :: SerializeSeq :: end (self) } }
    };
}

impl_197!();