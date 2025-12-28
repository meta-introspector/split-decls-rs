macro_rules! deps {
    () => {
        LocalModuleId!();
        DefMap!();
        LocalDefMap!();
    };
}

macro_rules! ModuleItemMap {
    () => {
        deps!();
        # [derive (Clone)] struct ModuleItemMap < 'db > { def_map : & 'db DefMap , local_def_map : & 'db LocalDefMap , module_id : LocalModuleId , }
    };
}

ModuleItemMap!();