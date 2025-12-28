macro_rules! deps {
    () => {
        WfCheckingCtxt!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl < 'a , 'tcx > Deref for WfCheckingCtxt < 'a , 'tcx > { type Target = ObligationCtxt < 'a , 'tcx , FulfillmentError < 'tcx > > ; fn deref (& self) -> & Self :: Target { & self . ocx } }
    };
}

impl_93!()