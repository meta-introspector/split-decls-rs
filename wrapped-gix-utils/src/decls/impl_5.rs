macro_rules! deps {
    () => {
        Quadratic!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl < Transform > Iterator for Quadratic < Transform > where Transform : Fn (usize) -> usize , { type Item = Duration ; fn next (& mut self) -> Option < Self :: Item > { let wait = Duration :: from_millis ((self . transform) (self . multiplier) as u64) ; self . multiplier += 2 * self . exponent + 1 ; if self . multiplier > self . max_multiplier { self . multiplier = self . max_multiplier ; } else { self . exponent += 1 ; } Some (wait) } }
    };
}

impl_5!();