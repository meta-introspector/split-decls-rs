macro_rules! deps {
    () => {
        SerdeEncoder!();
        EncodeError!();
        Compound!();
        Encoder!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        impl < ENC : Encoder > SerializeTuple for Compound < '_ , ENC > { type Ok = () ; type Error = EncodeError ; fn serialize_element < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : Serialize + ? Sized , { value . serialize (SerdeEncoder { enc : self . enc }) } fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (()) } }
    };
}

impl_169!();