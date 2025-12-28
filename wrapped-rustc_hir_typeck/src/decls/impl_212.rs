macro_rules! deps {
    () => {
        FnCtxt!();
        TypeckRootCtxt!();
    };
}

macro_rules! impl_212 {
    () => {
        deps!();
        impl < 'a , 'tcx > Deref for FnCtxt < 'a , 'tcx > { type Target = TypeckRootCtxt < 'tcx > ; fn deref (& self) -> & Self :: Target { self . root_ctxt } }
    };
}

impl_212!()