macro_rules! Diverges {
    () => {
        # [derive (Copy , Clone , Debug , PartialEq , Eq , PartialOrd , Ord)] enum Diverges { Maybe , Always , }
    };
}

Diverges!();