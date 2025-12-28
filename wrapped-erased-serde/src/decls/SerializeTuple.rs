macro_rules! deps {
    () => {
        Result!();
        ErrorImpl!();
        Serialize!();
    };
}

macro_rules! SerializeTuple {
    () => {
        deps!();
        pub trait SerializeTuple { fn erased_serialize_element (& mut self , value : & dyn Serialize) -> Result < () , ErrorImpl > ; fn erased_end (& mut self) ; }
    };
}

SerializeTuple!();