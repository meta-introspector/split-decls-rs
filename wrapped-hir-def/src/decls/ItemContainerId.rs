macro_rules! deps {
    () => {
        ModuleId!();
    };
}

macro_rules! ItemContainerId {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum ItemContainerId { ExternBlockId (ExternBlockId) , ModuleId (ModuleId) , ImplId (ImplId) , TraitId (TraitId) , }
    };
}

ItemContainerId!();