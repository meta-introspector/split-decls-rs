macro_rules! deps {
    () => {
        ModuleDef!();
        Macro!();
    };
}

macro_rules! ItemInNs {
    () => {
        deps!();
        # [derive (Clone , Copy , PartialEq , Eq , Debug , Hash)] pub enum ItemInNs { Types (ModuleDef) , Values (ModuleDef) , Macros (Macro) , }
    };
}

ItemInNs!()