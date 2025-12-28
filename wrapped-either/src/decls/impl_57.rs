macro_rules! deps {
    () => {
        Either!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < L , R > DerefMut for Either < L , R > where L : DerefMut , R : DerefMut < Target = L :: Target > , { fn deref_mut (& mut self) -> & mut Self :: Target { for_both ! (self , inner => & mut * inner) } }
    };
}

impl_57!();