macro_rules! deps {
    () => {
        Value!();
        SerializeVec!();
        Error!();
        Result!();
    };
}

macro_rules! impl_349 {
    () => {
        deps!();
        impl serde :: ser :: SerializeTuple for SerializeVec { type Ok = Value ; type Error = Error ; fn serialize_element < T > (& mut self , value : & T) -> Result < () > where T : ? Sized + Serialize , { serde :: ser :: SerializeSeq :: serialize_element (self , value) } fn end (self) -> Result < Value > { serde :: ser :: SerializeSeq :: end (self) } }
    };
}

impl_349!();