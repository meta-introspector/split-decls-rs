macro_rules! deps {
    () => {
        ErrorImpl!();
        Serialize!();
        Result!();
    };
}

macro_rules! SerializeTupleVariant {
    () => {
        deps!();
        pub trait SerializeTupleVariant { fn erased_serialize_field (& mut self , value : & dyn Serialize) -> Result < () , ErrorImpl > ; fn erased_end (& mut self) ; }
    };
}

SerializeTupleVariant!();