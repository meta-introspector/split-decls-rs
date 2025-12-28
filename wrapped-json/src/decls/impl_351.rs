macro_rules! deps {
    () => {
        Value!();
        Map!();
        Result!();
        SerializeTupleVariant!();
        Error!();
    };
}

macro_rules! impl_351 {
    () => {
        deps!();
        impl serde :: ser :: SerializeTupleVariant for SerializeTupleVariant { type Ok = Value ; type Error = Error ; fn serialize_field < T > (& mut self , value : & T) -> Result < () > where T : ? Sized + Serialize , { self . vec . push (tri ! (to_value (value))) ; Ok (()) } fn end (self) -> Result < Value > { let mut object = Map :: new () ; object . insert (self . name , Value :: Array (self . vec)) ; Ok (Value :: Object (object)) } }
    };
}

impl_351!();