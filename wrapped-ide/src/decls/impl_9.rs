macro_rules! deps {
    () => {
        Markup!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl From < String > for Markup { fn from (text : String) -> Self { Markup { text } } }
    };
}

impl_9!()