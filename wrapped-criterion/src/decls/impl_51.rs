macro_rules! deps {
    () => {
        MessageError!();
        Error!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl From < std :: io :: Error > for MessageError { fn from (other : std :: io :: Error) -> Self { MessageError :: Io (other) } }
    };
}

impl_51!();