macro_rules! deps {
    () => {
        SerializeStructVariant!();
        Error!();
        MakeSerializer!();
        Result!();
        Serialize!();
        ErrorImpl!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        impl serde :: ser :: SerializeStructVariant for MakeSerializer < & mut dyn SerializeStructVariant > { type Ok = () ; type Error = ErrorImpl ; fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () , Self :: Error > where T : ? Sized + serde :: Serialize , { self . 0 . erased_serialize_field (key , & value) } fn skip_field (& mut self , key : & 'static str) -> Result < () , Self :: Error > { self . 0 . erased_skip_field (key) } fn end (self) -> Result < Self :: Ok , Self :: Error > { self . 0 . erased_end () ; Ok (()) } }
    };
}

impl_116!();