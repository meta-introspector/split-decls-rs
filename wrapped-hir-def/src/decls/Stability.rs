macro_rules! Stability {
    () => {
        # [derive (Copy , Clone , Debug , PartialEq , Eq , PartialOrd , Ord)] enum Stability { Unstable , Stable , }
    };
}

Stability!()