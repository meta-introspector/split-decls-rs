macro_rules! deps {
    () => {
        FnCtxt!();
        ConfirmContext!();
    };
}

macro_rules! impl_244 {
    () => {
        deps!();
        impl < 'a , 'tcx > Deref for ConfirmContext < 'a , 'tcx > { type Target = FnCtxt < 'a , 'tcx > ; fn deref (& self) -> & Self :: Target { self . fcx } }
    };
}

impl_244!();