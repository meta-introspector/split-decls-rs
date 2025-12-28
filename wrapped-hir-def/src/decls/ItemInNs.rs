macro_rules! deps {
    () => {
        MacroId!();
        ModuleDefId!();
    };
}

macro_rules! ItemInNs {
    () => {
        deps!();
        # [derive (Clone , Copy , PartialEq , Eq , Debug , Hash)] pub enum ItemInNs { Types (ModuleDefId) , Values (ModuleDefId) , Macros (MacroId) , }
    };
}

ItemInNs!();