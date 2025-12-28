macro_rules! deps {
    () => {
        Encoder!();
        Compound!();
        SerdeEncoder!();
        EncodeError!();
    };
}

macro_rules! impl_174 {
    () => {
        deps!();
        impl < ENC : Encoder > SerializeStructVariant for Compound < '_ , ENC > { type Ok = () ; type Error = EncodeError ; fn serialize_field < T > (& mut self , _key : & 'static str , value : & T) -> Result < () , Self :: Error > where T : Serialize + ? Sized , { value . serialize (SerdeEncoder { enc : self . enc }) } fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (()) } }
    };
}

impl_174!()