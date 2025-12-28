macro_rules! deps {
    () => {
        ProbeContext!();
        FnCtxt!();
    };
}

macro_rules! impl_253 {
    () => {
        deps!();
        impl < 'a , 'tcx > Deref for ProbeContext < 'a , 'tcx > { type Target = FnCtxt < 'a , 'tcx > ; fn deref (& self) -> & Self :: Target { self . fcx } }
    };
}

impl_253!()