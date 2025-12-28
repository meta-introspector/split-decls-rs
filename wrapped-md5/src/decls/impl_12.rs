macro_rules! deps {
    () => {
        Context!();
        Digest!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl core :: convert :: From < Context > for Digest { # [inline] fn from (context : Context) -> Digest { context . finalize () } }
    };
}

impl_12!()