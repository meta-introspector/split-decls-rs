macro_rules! deps {
    () => {
        ScopedInterface!();
        Interface!();
    };
}

macro_rules! impl_194 {
    () => {
        deps!();
        impl < T : Interface > ScopedInterface < '_ , T > { pub fn new (interface : T) -> Self { Self { interface , lifetime : PhantomData , } } }
    };
}

impl_194!();