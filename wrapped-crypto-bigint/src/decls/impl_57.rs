macro_rules! deps {
    () => {
        ConstChoice!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl From < ConstChoice > for bool { fn from (choice : ConstChoice) -> Self { choice . is_true_vartime () } }
    };
}

impl_57!()