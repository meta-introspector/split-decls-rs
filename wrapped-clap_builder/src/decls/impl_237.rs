macro_rules! deps {
    () => {
        StyledStr!();
    };
}

macro_rules! impl_237 {
    () => {
        deps!();
        impl From < & 'static str > for StyledStr { fn from (name : & 'static str) -> Self { let mut styled = StyledStr :: new () ; styled . push_str (name) ; styled } }
    };
}

impl_237!()