macro_rules! deps {
    () => {
        Weight!();
    };
}

macro_rules! impl_weight_float {
    () => {
        deps!();
        macro_rules ! impl_weight_float { ($ t : ty) => { impl Weight for $ t { const ZERO : Self = 0.0 ; fn checked_add_assign (& mut self , v : & Self) -> Result < () , () > { * self += * v ; Ok (()) } } } ; }
    };
}

impl_weight_float!();