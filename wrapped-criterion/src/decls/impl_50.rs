macro_rules! deps {
    () => {
        MessageError!();
        Error!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl From < ciborium :: ser :: Error < std :: io :: Error > > for MessageError { fn from (other : ciborium :: ser :: Error < std :: io :: Error >) -> Self { MessageError :: Serialization (other) } }
    };
}

impl_50!();