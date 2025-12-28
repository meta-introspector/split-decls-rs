macro_rules! deps {
    () => {
        State!();
        Name!();
    };
}

macro_rules! Assignment {
    () => {
        deps!();
        # [doc = " Name an attribute and describe it's assigned state."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Assignment { # [doc = " The validated name of the attribute."] pub name : Name , # [doc = " The state of the attribute."] pub state : State , }
    };
}

Assignment!();