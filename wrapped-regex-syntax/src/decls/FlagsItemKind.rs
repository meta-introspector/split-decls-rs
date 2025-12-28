macro_rules! deps {
    () => {
        Flag!();
    };
}

macro_rules! FlagsItemKind {
    () => {
        deps!();
        # [doc = " The kind of an item in a group of flags."] # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub enum FlagsItemKind { # [doc = " A negation operator applied to all subsequent flags in the enclosing"] # [doc = " group."] Negation , # [doc = " A single flag in a group."] Flag (Flag) , }
    };
}

FlagsItemKind!();