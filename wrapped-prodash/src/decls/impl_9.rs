macro_rules! deps {
    () => {
        Item!();
        Id!();
        MessageLevel!();
        Step!();
        Progress!();
        Unit!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl crate :: Progress for Item { fn init (& mut self , max : Option < Step > , unit : Option < Unit >) { Item :: init (self , max , unit) } fn unit (& self) -> Option < Unit > { Item :: unit (self) } fn max (& self) -> Option < usize > { Item :: max (self) } fn set_max (& mut self , max : Option < Step >) -> Option < Step > { Item :: set_max (self , max) } fn set_name (& mut self , name : String) { Item :: set_name (self , name) } fn name (& self) -> Option < String > { Item :: name (self) } fn id (& self) -> Id { Item :: id (self) } fn message (& self , level : MessageLevel , message : String) { Item :: message (self , level , message) } }
    };
}

impl_9!();