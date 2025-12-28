macro_rules! deps {
    () => {
        Progress!();
        MessageLevel!();
        Step!();
        Either!();
        Unit!();
        Id!();
    };
}

macro_rules! impl_161 {
    () => {
        deps!();
        impl < L , R > Progress for Either < L , R > where L : Progress , R : Progress , { fn init (& mut self , max : Option < usize > , unit : Option < Unit >) { match self { Either :: Left (l) => l . init (max , unit) , Either :: Right (r) => r . init (max , unit) , } } fn unit (& self) -> Option < Unit > { match self { Either :: Left (l) => l . unit () , Either :: Right (r) => r . unit () , } } fn max (& self) -> Option < usize > { match self { Either :: Left (l) => l . max () , Either :: Right (r) => r . max () , } } fn set_max (& mut self , max : Option < Step >) -> Option < Step > { match self { Either :: Left (l) => l . set_max (max) , Either :: Right (r) => r . set_max (max) , } } fn set_name (& mut self , name : String) { match self { Either :: Left (l) => l . set_name (name) , Either :: Right (r) => r . set_name (name) , } } fn name (& self) -> Option < String > { match self { Either :: Left (l) => l . name () , Either :: Right (r) => r . name () , } } fn id (& self) -> Id { match self { Either :: Left (l) => l . id () , Either :: Right (r) => r . id () , } } fn message (& self , level : MessageLevel , message : String) { match self { Either :: Left (l) => l . message (level , message) , Either :: Right (r) => r . message (level , message) , } } }
    };
}

impl_161!();