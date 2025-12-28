macro_rules! deps {
    () => {
        DefMap!();
        ModuleData!();
    };
}

macro_rules! LocalModuleId {
    () => {
        deps!();
        # [doc = " An ID of a module, **local** to a `DefMap`."] pub type LocalModuleId = Idx < nameres :: ModuleData > ;
    };
}

LocalModuleId!()