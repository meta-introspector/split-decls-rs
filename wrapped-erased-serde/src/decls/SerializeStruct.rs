macro_rules! deps {
    () => {
        Serialize!();
        Result!();
        ErrorImpl!();
    };
}

macro_rules! SerializeStruct {
    () => {
        deps!();
        pub trait SerializeStruct { fn erased_serialize_field (& mut self , key : & 'static str , value : & dyn Serialize ,) -> Result < () , ErrorImpl > ; fn erased_skip_field (& mut self , key : & 'static str) -> Result < () , ErrorImpl > ; fn erased_end (& mut self) ; }
    };
}

SerializeStruct!();