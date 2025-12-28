macro_rules! deps {
    () => {
        Error!();
        MessageError!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl From < ciborium :: ser :: Error < std :: io :: Error > > for MessageError { fn from (other : ciborium :: ser :: Error < std :: io :: Error >) -> Self { MessageError :: Serialization (other) } }
    };
}

impl_50!()