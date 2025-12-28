macro_rules! deps {
    () => {
        ProcessingError!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl From < core :: fmt :: Error > for ProcessingError { fn from (_ : core :: fmt :: Error) -> Self { Self :: SinkError } }
    };
}

impl_71!();