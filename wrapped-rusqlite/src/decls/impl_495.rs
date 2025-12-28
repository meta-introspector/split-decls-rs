macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_495 {
    () => {
        deps!();
        impl From < String > for Value { # [inline] fn from (s : String) -> Self { Self :: Text (s) } }
    };
}

impl_495!()