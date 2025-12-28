macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! Kind {
    () => {
        deps!();
        # [doc = " The kind of attribute that was parsed."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub enum Kind { # [doc = " A pattern to match paths against"] Pattern (gix_glob :: Pattern) , # [doc = " The name of the macro to define, always a valid attribute name"] Macro (Name) , }
    };
}

Kind!()