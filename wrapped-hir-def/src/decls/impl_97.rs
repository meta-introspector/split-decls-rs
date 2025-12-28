macro_rules! deps {
    () => {
        CrateRootModuleId!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl From < Crate > for CrateRootModuleId { fn from (krate : Crate) -> Self { CrateRootModuleId { krate } } }
    };
}

impl_97!()