macro_rules! deps {
    () => {
        Access!();
        Guard!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl < T , A : Access < T > + ? Sized , P : Deref < Target = A > > Access < T > for P { type Guard = A :: Guard ; fn load (& self) -> Self :: Guard { self . deref () . load () } }
    };
}

impl_2!()