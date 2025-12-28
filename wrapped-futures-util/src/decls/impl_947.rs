macro_rules! impl_947 {
    () => {
        impl < Si , F > SinkMapErr < Si , F > { pub (super) fn new (sink : Si , f : F) -> Self { Self { sink , f : Some (f) } } delegate_access_inner ! (sink , Si , ()) ; fn take_f (self : Pin < & mut Self >) -> F { self . project () . f . take () . expect ("polled MapErr after completion") } }
    };
}

impl_947!()