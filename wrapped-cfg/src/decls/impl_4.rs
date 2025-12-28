macro_rules! deps {
    () => {
        CfgOptions!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl Default for CfgOptions { fn default () -> Self { Self { enabled : FxHashSet :: from_iter ([CfgAtom :: Flag (sym :: true_)]) } } }
    };
}

impl_4!()