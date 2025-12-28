macro_rules! deps {
    () => {
        ModuleId!();
        DefMap!();
        CrateRootModuleId!();
    };
}

macro_rules! impl_665 {
    () => {
        deps!();
        impl PartialEq < ModuleId > for CrateRootModuleId { fn eq (& self , other : & ModuleId) -> bool { other . block . is_none () && other . local_id == DefMap :: ROOT && self . krate == other . krate } }
    };
}

impl_665!()