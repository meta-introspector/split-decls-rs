macro_rules! deps {
    () => {
        Result!();
        ErrorImpl!();
        Serialize!();
    };
}

macro_rules! SerializeTupleStruct {
    () => {
        deps!();
        pub trait SerializeTupleStruct { fn erased_serialize_field (& mut self , value : & dyn Serialize) -> Result < () , ErrorImpl > ; fn erased_end (& mut self) ; }
    };
}

SerializeTupleStruct!();