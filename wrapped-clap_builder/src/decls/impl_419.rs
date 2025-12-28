macro_rules! deps {
    () => {
        Message!();
        StyledStr!();
    };
}

macro_rules! impl_419 {
    () => {
        deps!();
        impl From < StyledStr > for Message { fn from (inner : StyledStr) -> Self { Self :: Formatted (inner) } }
    };
}

impl_419!();