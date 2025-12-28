macro_rules! deps {
    () => {
        Serialize!();
        Result!();
        ErrorImpl!();
    };
}

macro_rules! SerializeSeq {
    () => {
        deps!();
        pub trait SerializeSeq { fn erased_serialize_element (& mut self , value : & dyn Serialize) -> Result < () , ErrorImpl > ; fn erased_end (& mut self) ; }
    };
}

SerializeSeq!()