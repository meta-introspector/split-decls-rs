macro_rules! deps {
    () => {
        Result!();
        Value!();
        SerializeVec!();
        Error!();
    };
}

macro_rules! impl_350 {
    () => {
        deps!();
        impl serde :: ser :: SerializeTupleStruct for SerializeVec { type Ok = Value ; type Error = Error ; fn serialize_field < T > (& mut self , value : & T) -> Result < () > where T : ? Sized + Serialize , { serde :: ser :: SerializeSeq :: serialize_element (self , value) } fn end (self) -> Result < Value > { serde :: ser :: SerializeSeq :: end (self) } }
    };
}

impl_350!();