macro_rules! deps {
    () => {
        Encoder!();
        SerdeEncoder!();
        Compound!();
        EncodeError!();
    };
}

macro_rules! impl_172 {
    () => {
        deps!();
        impl < ENC : Encoder > SerializeMap for Compound < '_ , ENC > { type Ok = () ; type Error = EncodeError ; fn serialize_key < T > (& mut self , key : & T) -> Result < () , Self :: Error > where T : Serialize + ? Sized , { key . serialize (SerdeEncoder { enc : self . enc }) } fn serialize_value < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : Serialize + ? Sized , { value . serialize (SerdeEncoder { enc : self . enc }) } fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (()) } }
    };
}

impl_172!();