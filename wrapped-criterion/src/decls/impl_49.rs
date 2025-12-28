macro_rules! deps {
    () => {
        MessageError!();
        Error!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl From < ciborium :: de :: Error < std :: io :: Error > > for MessageError { fn from (other : ciborium :: de :: Error < std :: io :: Error >) -> Self { MessageError :: Deserialization (other) } }
    };
}

impl_49!()