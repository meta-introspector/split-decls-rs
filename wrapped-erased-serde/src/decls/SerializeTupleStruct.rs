macro_rules! deps {
    () => {
        Serialize!();
        ErrorImpl!();
        Result!();
    };
}

macro_rules! SerializeTupleStruct {
    () => {
        deps!();
        pub trait SerializeTupleStruct { fn erased_serialize_field (& mut self , value : & dyn Serialize) -> Result < () , ErrorImpl > ; fn erased_end (& mut self) ; }
    };
}

SerializeTupleStruct!()