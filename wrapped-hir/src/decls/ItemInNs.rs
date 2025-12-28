macro_rules! deps {
    () => {
        Macro!();
        ModuleDef!();
    };
}

macro_rules! ItemInNs {
    () => {
        deps!();
        # [derive (Clone , Copy , PartialEq , Eq , Debug , Hash)] pub enum ItemInNs { Types (ModuleDef) , Values (ModuleDef) , Macros (Macro) , }
    };
}

ItemInNs!();