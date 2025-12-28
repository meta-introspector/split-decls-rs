macro_rules! deps {
    () => {
        ErrorImpl!();
        Serialize!();
        Result!();
    };
}

macro_rules! SerializeMap {
    () => {
        deps!();
        pub trait SerializeMap { fn erased_serialize_key (& mut self , key : & dyn Serialize) -> Result < () , ErrorImpl > ; fn erased_serialize_value (& mut self , value : & dyn Serialize) -> Result < () , ErrorImpl > ; fn erased_serialize_entry (& mut self , key : & dyn Serialize , value : & dyn Serialize ,) -> Result < () , ErrorImpl > ; fn erased_end (& mut self) ; }
    };
}

SerializeMap!()