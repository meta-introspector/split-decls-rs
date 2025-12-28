macro_rules! deps {
    () => {
        Unit!();
        Discard!();
        Id!();
        Progress!();
        Step!();
        MessageLevel!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        impl Progress for Discard { fn init (& mut self , _max : Option < usize > , _unit : Option < Unit >) { } fn set_max (& mut self , _max : Option < Step >) -> Option < Step > { None } fn set_name (& mut self , _name : String) { } fn name (& self) -> Option < String > { None } fn id (& self) -> Id { crate :: progress :: UNKNOWN } fn message (& self , _level : MessageLevel , _message : String) { } }
    };
}

impl_157!();