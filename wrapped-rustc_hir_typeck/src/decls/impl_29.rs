macro_rules! deps {
    () => {
        Coerce!();
        FnCtxt!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < 'a , 'tcx > Deref for Coerce < 'a , 'tcx > { type Target = FnCtxt < 'a , 'tcx > ; fn deref (& self) -> & Self :: Target { self . fcx } }
    };
}

impl_29!()