macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_940 {
    () => {
        deps!();
        impl gix_protocol :: transport :: IsSpuriousError for Error { fn is_spurious (& self) -> bool { match self { Error :: Transport (err) => err . is_spurious () , Error :: Handshake (err) => err . is_spurious () , _ => false , } } }
    };
}

impl_940!();