macro_rules! deps {
    () => {
        FnCtxt!();
        Coerce!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < 'a , 'tcx > Deref for Coerce < 'a , 'tcx > { type Target = FnCtxt < 'a , 'tcx > ; fn deref (& self) -> & Self :: Target { self . fcx } }
    };
}

impl_29!();