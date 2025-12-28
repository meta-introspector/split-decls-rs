macro_rules! deps {
    () => {
        StyledStr!();
    };
}

macro_rules! impl_238 {
    () => {
        deps!();
        impl From < & '_ & 'static str > for StyledStr { fn from (name : & '_ & 'static str) -> Self { StyledStr :: from (* name) } }
    };
}

impl_238!();