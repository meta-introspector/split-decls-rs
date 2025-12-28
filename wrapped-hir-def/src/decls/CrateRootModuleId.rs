macro_rules! deps {
    () => {
        ModuleId!();
    };
}

macro_rules! CrateRootModuleId {
    () => {
        deps!();
        # [doc = " A `ModuleId` that is always a crate's root module."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct CrateRootModuleId { krate : Crate , }
    };
}

CrateRootModuleId!();