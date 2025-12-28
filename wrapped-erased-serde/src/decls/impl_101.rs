macro_rules! deps {
    () => {
        SerializeTuple!();
        MakeSerializer!();
        Serialize!();
        Result!();
        ErrorImpl!();
        Error!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl serde :: ser :: SerializeTuple for MakeSerializer < & mut dyn SerializeTuple > { type Ok = () ; type Error = ErrorImpl ; fn serialize_element < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : ? Sized + serde :: Serialize , { self . 0 . erased_serialize_element (& value) } fn end (self) -> Result < Self :: Ok , Self :: Error > { self . 0 . erased_end () ; Ok (()) } }
    };
}

impl_101!()