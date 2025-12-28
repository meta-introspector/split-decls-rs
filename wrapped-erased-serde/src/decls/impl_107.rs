macro_rules! deps {
    () => {
        Result!();
        SerializeTupleVariant!();
        MakeSerializer!();
        ErrorImpl!();
        Serialize!();
        Error!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl serde :: ser :: SerializeTupleVariant for MakeSerializer < & mut dyn SerializeTupleVariant > { type Ok = () ; type Error = ErrorImpl ; fn serialize_field < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : ? Sized + serde :: Serialize , { self . 0 . erased_serialize_field (& value) } fn end (self) -> Result < Self :: Ok , Self :: Error > { self . 0 . erased_end () ; Ok (()) } }
    };
}

impl_107!()