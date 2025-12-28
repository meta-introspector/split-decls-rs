macro_rules! deps {
    () => {
        MemoryUsage!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl std :: ops :: Sub for MemoryUsage { type Output = MemoryUsage ; fn sub (self , rhs : MemoryUsage) -> MemoryUsage { MemoryUsage { allocated : self . allocated - rhs . allocated } } }
    };
}

impl_10!()