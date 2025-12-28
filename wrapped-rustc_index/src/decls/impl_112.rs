macro_rules! deps {
    () => {
        IndexVec!();
        Idx!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl < I : Idx , T > DerefMut for IndexVec < I , T > { # [inline] fn deref_mut (& mut self) -> & mut Self :: Target { self . as_mut_slice () } }
    };
}

impl_112!();