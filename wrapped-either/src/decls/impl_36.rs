macro_rules! deps {
    () => {
        Either!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl < L , R > Deref for Either < L , R > where L : Deref , R : Deref < Target = L :: Target > , { type Target = L :: Target ; fn deref (& self) -> & Self :: Target { for_both ! (self , inner => &** inner) } }
    };
}

impl_36!()