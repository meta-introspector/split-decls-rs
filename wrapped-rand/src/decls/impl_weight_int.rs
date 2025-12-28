macro_rules! deps {
    () => {
        Weight!();
    };
}

macro_rules! impl_weight_int {
    () => {
        deps!();
        macro_rules ! impl_weight_int { ($ t : ty) => { impl Weight for $ t { const ZERO : Self = 0 ; fn checked_add_assign (& mut self , v : & Self) -> Result < () , () > { match self . checked_add (* v) { Some (sum) => { * self = sum ; Ok (()) } None => Err (()) , } } } } ; ($ t : ty , $ ($ tt : ty) ,*) => { impl_weight_int ! ($ t) ; impl_weight_int ! ($ ($ tt) ,*) ; } }
    };
}

impl_weight_int!();