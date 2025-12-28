macro_rules! deps {
    () => {
        DummyMachine!();
        HasStaticRootDefId!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl HasStaticRootDefId for DummyMachine { fn static_def_id (& self) -> Option < rustc_hir :: def_id :: LocalDefId > { None } }
    };
}

impl_84!();