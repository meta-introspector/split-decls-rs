macro_rules! Local {
    () => {
        # [derive (Debug , Clone , PartialEq , Eq)] pub struct Local < 'db > { pub ty : Ty < 'db > , }
    };
}

Local!()