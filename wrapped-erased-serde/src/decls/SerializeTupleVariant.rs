macro_rules! deps {
    () => {
        Serialize!();
        ErrorImpl!();
        Result!();
    };
}

macro_rules! SerializeTupleVariant {
    () => {
        deps!();
        pub trait SerializeTupleVariant { fn erased_serialize_field (& mut self , value : & dyn Serialize) -> Result < () , ErrorImpl > ; fn erased_end (& mut self) ; }
    };
}

SerializeTupleVariant!()