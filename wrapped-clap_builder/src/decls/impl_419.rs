macro_rules! deps {
    () => {
        StyledStr!();
        Message!();
    };
}

macro_rules! impl_419 {
    () => {
        deps!();
        impl From < StyledStr > for Message { fn from (inner : StyledStr) -> Self { Self :: Formatted (inner) } }
    };
}

impl_419!()