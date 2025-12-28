macro_rules! deps {
    () => {
        Checker!();
        ConstCx!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl < 'mir , 'tcx > Deref for Checker < 'mir , 'tcx > { type Target = ConstCx < 'mir , 'tcx > ; fn deref (& self) -> & Self :: Target { self . ccx } }
    };
}

impl_5!();