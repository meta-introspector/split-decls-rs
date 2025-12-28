macro_rules! deps {
    () => {
        DiffActivity!();
        Const!();
    };
}

macro_rules! impl_312 {
    () => {
        deps!();
        impl DiffActivity { pub fn is_dual_or_const (& self) -> bool { use DiffActivity :: * ; matches ! (self , | Dual | DualOnly | Dualv | DualvOnly | Const) } }
    };
}

impl_312!();