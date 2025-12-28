macro_rules! deps {
    () => {
        Spacing!();
        DelimSpacing!();
    };
}

macro_rules! impl_460 {
    () => {
        deps!();
        impl DelimSpacing { pub fn new (open : Spacing , close : Spacing) -> DelimSpacing { DelimSpacing { open , close } } }
    };
}

impl_460!()