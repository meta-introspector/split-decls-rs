macro_rules! deps {
    () => {
        Error!();
        SerializeVec!();
        Result!();
        Value!();
    };
}

macro_rules! impl_348 {
    () => {
        deps!();
        impl serde :: ser :: SerializeSeq for SerializeVec { type Ok = Value ; type Error = Error ; fn serialize_element < T > (& mut self , value : & T) -> Result < () > where T : ? Sized + Serialize , { self . vec . push (tri ! (to_value (value))) ; Ok (()) } fn end (self) -> Result < Value > { Ok (Value :: Array (self . vec)) } }
    };
}

impl_348!()