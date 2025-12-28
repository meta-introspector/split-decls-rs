macro_rules! deps {
    () => {
        Type!();
    };
}

macro_rules! Param {
    () => {
        deps!();
        # [derive (Clone , Debug , PartialEq , Eq)] pub struct Param { pub def : MethodParam , pub ty : Type , }
    };
}

Param!()