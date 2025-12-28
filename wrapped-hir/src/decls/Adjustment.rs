macro_rules! deps {
    () => {
        Type!();
        Adjust!();
    };
}

macro_rules! Adjustment {
    () => {
        deps!();
        # [derive (Clone , Debug , PartialEq , Eq)] pub struct Adjustment < 'db > { pub source : Type < 'db > , pub target : Type < 'db > , pub kind : Adjust , }
    };
}

Adjustment!()