macro_rules! deps {
    () => {
        LocalModuleId!();
    };
}

macro_rules! PerNsGlobImports {
    () => {
        deps!();
        # [derive (Debug , Default)] pub struct PerNsGlobImports { types : FxHashSet < (LocalModuleId , Name) > , values : FxHashSet < (LocalModuleId , Name) > , macros : FxHashSet < (LocalModuleId , Name) > , }
    };
}

PerNsGlobImports!();