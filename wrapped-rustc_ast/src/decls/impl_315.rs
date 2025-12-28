macro_rules! deps {
    () => {
        AutoDiffAttrs!();
        DiffActivity!();
    };
}

macro_rules! impl_315 {
    () => {
        deps!();
        impl AutoDiffAttrs { pub fn has_primal_ret (& self) -> bool { matches ! (self . ret_activity , DiffActivity :: Active | DiffActivity :: Dual) } }
    };
}

impl_315!()