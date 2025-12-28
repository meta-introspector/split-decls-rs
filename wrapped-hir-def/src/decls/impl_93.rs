macro_rules! deps {
    () => {
        CrateRootModuleId!();
        ModuleId!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl PartialEq < ModuleId > for CrateRootModuleId { fn eq (& self , other : & ModuleId) -> bool { other . block . is_none () && other . local_id == DefMap :: ROOT && self . krate == other . krate } }
    };
}

impl_93!()