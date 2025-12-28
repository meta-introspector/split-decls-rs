macro_rules! deps {
    () => {
        Id!();
        Step!();
        Unit!();
        DoOrDiscard!();
        MessageLevel!();
        Progress!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl < T > Progress for DoOrDiscard < T > where T : Progress , { fn init (& mut self , max : Option < usize > , unit : Option < Unit >) { self . 0 . init (max , unit) } fn unit (& self) -> Option < Unit > { self . 0 . unit () } fn max (& self) -> Option < usize > { self . 0 . max () } fn set_max (& mut self , max : Option < Step >) -> Option < Step > { self . 0 . set_max (max) } fn set_name (& mut self , name : String) { self . 0 . set_name (name) ; } fn name (& self) -> Option < String > { self . 0 . name () } fn id (& self) -> Id { self . 0 . id () } fn message (& self , level : MessageLevel , message : String) { self . 0 . message (level , message) } }
    };
}

impl_167!();