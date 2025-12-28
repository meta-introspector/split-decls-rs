macro_rules! deps {
    () => {
        SerializeSeq!();
        Result!();
        Error!();
        MakeSerializer!();
        ErrorImpl!();
        Serialize!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl serde :: ser :: SerializeSeq for MakeSerializer < & mut dyn SerializeSeq > { type Ok = () ; type Error = ErrorImpl ; fn serialize_element < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : ? Sized + serde :: Serialize , { self . 0 . erased_serialize_element (& value) } fn end (self) -> Result < Self :: Ok , Self :: Error > { self . 0 . erased_end () ; Ok (()) } }
    };
}

impl_98!();