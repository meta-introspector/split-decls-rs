macro_rules! deps {
    () => {
        StyledStr!();
    };
}

macro_rules! impl_235 {
    () => {
        deps!();
        impl From < String > for StyledStr { fn from (name : String) -> Self { StyledStr (name) } }
    };
}

impl_235!();