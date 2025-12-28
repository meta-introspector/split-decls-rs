macro_rules! deps {
    () => {
        Type!();
        Callee!();
    };
}

macro_rules! Param {
    () => {
        deps!();
        # [derive (Clone , PartialEq , Eq , Hash , Debug)] pub struct Param < 'db > { func : Callee < 'db > , # [doc = " The index in parameter list, including self parameter."] idx : usize , ty : Type < 'db > , }
    };
}

Param!()