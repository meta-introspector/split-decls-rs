macro_rules! deps {
    () => {
        OptimizeAttr!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl OptimizeAttr { pub fn do_not_optimize (& self) -> bool { matches ! (self , Self :: DoNotOptimize) } }
    };
}

impl_7!();