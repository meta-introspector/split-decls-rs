macro_rules! deps {
    () => {
        IntoConcurrentStream!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl < T > concurrent_stream :: IntoConcurrentStream for Vec < T > { type Item = T ; type IntoConcurrentStream = IntoConcurrentStream < T > ; fn into_co_stream (self) -> Self :: IntoConcurrentStream { let stream = from_iter (self) ; let co_stream = stream . co () ; IntoConcurrentStream (co_stream) } }
    };
}

impl_3!()