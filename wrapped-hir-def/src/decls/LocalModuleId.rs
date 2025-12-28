macro_rules! LocalModuleId {
    () => {
        # [doc = " An ID of a module, **local** to a `DefMap`."] pub type LocalModuleId = Idx < nameres :: ModuleData > ;
    };
}

LocalModuleId!()