macro_rules! deps {
    () => {
        StyledStr!();
    };
}

macro_rules! impl_236 {
    () => {
        deps!();
        impl From < & '_ String > for StyledStr { fn from (name : & '_ String) -> Self { let mut styled = StyledStr :: new () ; styled . push_str (name) ; styled } }
    };
}

impl_236!();