macro_rules! deps {
    () => {
        WorkerHandle!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl < T > Clone for WorkerHandle < T > { fn clone (& self) -> Self { Self { inner : Arc :: clone (& self . inner) , } } }
    };
}

impl_127!();