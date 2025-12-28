macro_rules! deps {
    () => {
        Encoder!();
        EncodeError!();
        SerdeEncoder!();
        Compound!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        impl < ENC : Encoder > SerializeTupleVariant for Compound < '_ , ENC > { type Ok = () ; type Error = EncodeError ; fn serialize_field < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : Serialize + ? Sized , { value . serialize (SerdeEncoder { enc : self . enc }) } fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (()) } }
    };
}

impl_171!();