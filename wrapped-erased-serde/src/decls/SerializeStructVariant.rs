macro_rules! deps {
    () => {
        Serialize!();
        ErrorImpl!();
        Result!();
    };
}

macro_rules! SerializeStructVariant {
    () => {
        deps!();
        pub trait SerializeStructVariant { fn erased_serialize_field (& mut self , key : & 'static str , value : & dyn Serialize ,) -> Result < () , ErrorImpl > ; fn erased_skip_field (& mut self , key : & 'static str) -> Result < () , ErrorImpl > ; fn erased_end (& mut self) ; }
    };
}

SerializeStructVariant!();