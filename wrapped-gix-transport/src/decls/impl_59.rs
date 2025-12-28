macro_rules! deps {
    () => {
        Error!();
        Curl!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl crate :: IsSpuriousError for Error { fn is_spurious (& self) -> bool { match self { Error :: Curl (err) => curl_is_spurious (err) , _ => false , } } }
    };
}

impl_59!()