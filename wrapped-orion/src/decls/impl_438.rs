macro_rules! deps {
    () => {
        RingElementNTT!();
    };
}

macro_rules! impl_438 {
    () => {
        deps!();
        impl AddAssign for RingElementNTT { fn add_assign (& mut self , other : Self) { add_poly (& self . coefficients . clone () , & other . coefficients , & mut self . coefficients ,) ; } }
    };
}

impl_438!();