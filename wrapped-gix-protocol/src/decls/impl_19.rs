macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl gix_transport :: IsSpuriousError for Error { fn is_spurious (& self) -> bool { match self { Error :: Io (err) => err . is_spurious () , Error :: Transport (err) => err . is_spurious () , _ => false , } } }
    };
}

impl_19!()