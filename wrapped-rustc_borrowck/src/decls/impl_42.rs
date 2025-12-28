macro_rules! deps {
    () => {
        BorrowckInferCtxt!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl < 'tcx > Deref for BorrowckInferCtxt < 'tcx > { type Target = InferCtxt < 'tcx > ; fn deref (& self) -> & Self :: Target { & self . infcx } }
    };
}

impl_42!()