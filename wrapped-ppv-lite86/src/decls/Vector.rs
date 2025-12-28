macro_rules! Vector {
    () => {
        pub trait Vector < T > { fn to_scalars (self) -> T ; }
    };
}

Vector!();