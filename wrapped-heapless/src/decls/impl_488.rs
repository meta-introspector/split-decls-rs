macro_rules! deps {
    () => {
        CapacityError!();
    };
}

macro_rules! impl_488 {
    () => {
        deps!();
        impl Error for CapacityError { fn kind (& self) -> ErrorKind { ErrorKind :: OutOfMemory } }
    };
}

impl_488!()