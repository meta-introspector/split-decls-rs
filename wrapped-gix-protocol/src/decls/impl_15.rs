macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl crate :: transport :: IsSpuriousError for Error { fn is_spurious (& self) -> bool { match self { Error :: FetchResponse (err) => err . is_spurious () , Error :: Client (err) => err . is_spurious () , _ => false , } } }
    };
}

impl_15!();