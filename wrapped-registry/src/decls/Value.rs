macro_rules! deps {
    () => {
        Data!();
    };
}

macro_rules! Value {
    () => {
        deps!();
        # [doc = " A registry value."] # [derive (Clone , PartialEq , Eq , Debug)] pub struct Value { pub (crate) data : Data , pub (crate) ty : Type , }
    };
}

Value!();