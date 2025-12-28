macro_rules! deps {
    () => {
        Message!();
    };
}

macro_rules! impl_418 {
    () => {
        deps!();
        impl From < String > for Message { fn from (inner : String) -> Self { Self :: Raw (inner) } }
    };
}

impl_418!()