macro_rules! deps {
    () => {
        StringNumber!();
    };
}

macro_rules! impl_809 {
    () => {
        deps!();
        impl < T : Num + Display + Default > Default for StringNumber < T > { # [inline] fn default () -> Self { Self (Default :: default ()) } }
    };
}

impl_809!();