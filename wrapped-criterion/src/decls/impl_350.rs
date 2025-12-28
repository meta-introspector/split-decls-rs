macro_rules! deps {
    () => {
        Bandwidth!();
        Sample!();
        Float!();
    };
}

macro_rules! impl_350 {
    () => {
        deps!();
        impl Bandwidth { fn estimate < A : Float > (self , sample : & Sample < A >) -> A { match self { Bandwidth :: Silverman => { let factor = A :: cast (4. / 3.) ; let exponent = A :: cast (1. / 5.) ; let n = A :: cast (sample . len ()) ; let sigma = sample . std_dev (None) ; sigma * (factor / n) . powf (exponent) } } } }
    };
}

impl_350!()