macro_rules! deps {
    () => {
        TypeckRootCtxt!();
    };
}

macro_rules! impl_337 {
    () => {
        deps!();
        impl < 'tcx > Deref for TypeckRootCtxt < 'tcx > { type Target = InferCtxt < 'tcx > ; fn deref (& self) -> & Self :: Target { & self . infcx } }
    };
}

impl_337!()