macro_rules! deps {
    () => {
        DefMap!();
        LocalModuleId!();
    };
}

macro_rules! ModuleId {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash , PartialOrd , Ord)] pub struct ModuleId { krate : Crate , # [doc = " If this `ModuleId` was derived from a `DefMap` for a block expression, this stores the"] # [doc = " `BlockId` of that block expression. If `None`, this module is part of the crate-level"] # [doc = " `DefMap` of `krate`."] block : Option < BlockId > , # [doc = " The module's ID in its originating `DefMap`."] pub local_id : LocalModuleId , }
    };
}

ModuleId!();