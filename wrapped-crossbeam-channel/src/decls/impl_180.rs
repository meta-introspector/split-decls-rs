macro_rules! deps {
    () => {
        Select!();
    };
}

macro_rules! impl_180 {
    () => {
        deps!();
        impl Clone for Select < '_ > { fn clone (& self) -> Self { Self { handles : self . handles . clone () , next_index : self . next_index , biased : self . biased , } } }
    };
}

impl_180!();