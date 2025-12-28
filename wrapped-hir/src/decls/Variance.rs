macro_rules! Variance {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum Variance { Bivariant , Covariant , Contravariant , Invariant , }
    };
}

Variance!()