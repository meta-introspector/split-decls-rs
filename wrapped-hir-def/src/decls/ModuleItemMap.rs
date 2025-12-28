macro_rules! deps {
    () => {
        DefMap!();
        LocalDefMap!();
        LocalModuleId!();
    };
}

macro_rules! ModuleItemMap {
    () => {
        deps!();
        # [derive (Clone)] struct ModuleItemMap < 'db > { def_map : & 'db DefMap , local_def_map : & 'db LocalDefMap , module_id : LocalModuleId , }
    };
}

ModuleItemMap!()