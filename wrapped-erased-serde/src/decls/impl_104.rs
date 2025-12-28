macro_rules! deps {
    () => {
        Result!();
        MakeSerializer!();
        Serialize!();
        SerializeTupleStruct!();
        ErrorImpl!();
        Error!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl serde :: ser :: SerializeTupleStruct for MakeSerializer < & mut dyn SerializeTupleStruct > { type Ok = () ; type Error = ErrorImpl ; fn serialize_field < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : ? Sized + serde :: Serialize , { self . 0 . erased_serialize_field (& value) } fn end (self) -> Result < Self :: Ok , Self :: Error > { self . 0 . erased_end () ; Ok (()) } }
    };
}

impl_104!();