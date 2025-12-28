macro_rules! deps {
    () => {
        ValueResult!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl < T , E > ValueResult < T , E > { pub fn new (value : T , err : E) -> Self { Self { value , err : Some (err) } } pub fn ok (value : T) -> Self { Self { value , err : None } } pub fn only_err (err : E) -> Self where T : Default , { Self { value : Default :: default () , err : Some (err) } } pub fn zip_val < U > (self , other : U) -> ValueResult < (T , U) , E > { ValueResult { value : (self . value , other) , err : self . err } } pub fn map < U > (self , f : impl FnOnce (T) -> U) -> ValueResult < U , E > { ValueResult { value : f (self . value) , err : self . err } } pub fn map_err < E2 > (self , f : impl FnOnce (E) -> E2) -> ValueResult < T , E2 > { ValueResult { value : self . value , err : self . err . map (f) } } pub fn result (self) -> Result < T , E > { self . err . map_or (Ok (self . value) , Err) } }
    };
}

impl_62!()