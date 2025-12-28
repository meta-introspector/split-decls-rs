macro_rules! PunycodeEncodeError {
    () => {
        pub (crate) enum PunycodeEncodeError { Overflow , Sink , }
    };
}

PunycodeEncodeError!()