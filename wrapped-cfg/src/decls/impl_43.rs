macro_rules! deps {
    () => {
        CfgAtom!();
        CfgOptions!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl Default for CfgOptions { fn default () -> Self { Self { enabled : FxHashSet :: from_iter ([CfgAtom :: Flag (sym :: true_)]) } } }
    };
}

impl_43!()