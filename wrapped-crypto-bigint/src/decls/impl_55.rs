macro_rules! deps {
    () => {
        ConstChoice!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl From < ConstChoice > for Choice { # [inline] fn from (choice : ConstChoice) -> Self { Choice :: from (choice . to_u8 ()) } }
    };
}

impl_55!()