macro_rules! deps {
    () => {
        Fetch!();
        Error!();
    };
}

macro_rules! impl_946 {
    () => {
        deps!();
        impl gix_protocol :: transport :: IsSpuriousError for Error { fn is_spurious (& self) -> bool { match self { Error :: Fetch (err) => err . is_spurious () , Error :: Client (err) => err . is_spurious () , _ => false , } } }
    };
}

impl_946!()