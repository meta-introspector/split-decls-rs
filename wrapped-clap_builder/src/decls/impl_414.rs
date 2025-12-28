macro_rules! deps {
    () => {
        Error!();
        ErrorFormatter!();
    };
}

macro_rules! impl_414 {
    () => {
        deps!();
        impl < F : ErrorFormatter > error :: Error for Error < F > { # [allow (trivial_casts)] fn source (& self) -> Option < & (dyn error :: Error + 'static) > { self . inner . source . as_ref () . map (| e | e . as_ref () as _) } }
    };
}

impl_414!()