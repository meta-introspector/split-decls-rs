macro_rules! deps {
    () => {
        Pointable!();
        Shared!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl < T : ? Sized + Pointable > Copy for Shared < '_ , T > { }
    };
}

impl_51!()