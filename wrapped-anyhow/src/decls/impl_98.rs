macro_rules! deps {
    () => {
        Trait!();
        Error!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl Trait { # [cold] pub fn new < E > (self , error : E) -> Error where E : Into < Error > , { error . into () } }
    };
}

impl_98!()