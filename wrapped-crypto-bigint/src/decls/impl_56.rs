macro_rules! deps {
    () => {
        ConstChoice!();
        Word!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl From < Choice > for ConstChoice { # [inline] fn from (choice : Choice) -> Self { ConstChoice :: from_word_lsb (choice . unwrap_u8 () as Word) } }
    };
}

impl_56!();