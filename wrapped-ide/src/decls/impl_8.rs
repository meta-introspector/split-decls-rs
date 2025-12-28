macro_rules! deps {
    () => {
        Markup!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl From < Markup > for String { fn from (markup : Markup) -> Self { markup . text } }
    };
}

impl_8!();