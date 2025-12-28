macro_rules! deps {
    () => {
        Uint!();
        Limb!();
    };
}

macro_rules! impl_374 {
    () => {
        deps!();
        impl < const LIMBS : usize > ConditionallySelectable for Uint < LIMBS > { fn conditional_select (a : & Self , b : & Self , choice : Choice) -> Self { let mut limbs = [Limb :: ZERO ; LIMBS] ; for i in 0 .. LIMBS { limbs [i] = Limb :: conditional_select (& a . limbs [i] , & b . limbs [i] , choice) ; } Self { limbs } } }
    };
}

impl_374!();