macro_rules! deps {
    () => {
        Compound!();
        SerdeEncoder!();
        EncodeError!();
        Encoder!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        impl < ENC : Encoder > SerializeSeq for Compound < '_ , ENC > { type Ok = () ; type Error = EncodeError ; fn serialize_element < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : Serialize + ? Sized , { value . serialize (SerdeEncoder { enc : self . enc }) } fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (()) } }
    };
}

impl_168!()