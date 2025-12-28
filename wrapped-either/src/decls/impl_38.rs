macro_rules! deps {
    () => {
        Either!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl < L , R > Either < & L , & R > { # [doc = " Maps an `Either<&L, &R>` to an `Either<L, R>` by cloning the contents of"] # [doc = " either branch."] pub fn cloned (self) -> Either < L , R > where L : Clone , R : Clone , { map_either ! (self , inner => inner . clone ()) } # [doc = " Maps an `Either<&L, &R>` to an `Either<L, R>` by copying the contents of"] # [doc = " either branch."] pub fn copied (self) -> Either < L , R > where L : Copy , R : Copy , { map_either ! (self , inner => * inner) } }
    };
}

impl_38!();