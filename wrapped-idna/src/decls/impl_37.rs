macro_rules! deps {
    () => {
        PunycodeEncodeError!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl From < core :: fmt :: Error > for PunycodeEncodeError { fn from (_ : core :: fmt :: Error) -> Self { Self :: Sink } }
    };
}

impl_37!();