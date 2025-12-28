macro_rules! deps {
    () => {
        Secret!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < T , E > Secret < Result < T , E > > { # [doc = " Converts a `Secret<Result<T, E>>` to a `Result<Secret<T>, E>`."] pub fn transpose (self) -> Result < Secret < T > , E > { self . inner . map (| v | Secret :: from (v)) } }
    };
}

impl_14!();