macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl crate :: IsSpuriousError for Error { fn is_spurious (& self) -> bool { match self { Error :: Reqwest (err) => { err . is_timeout () || err . is_connect () || err . status () . is_some_and (| status | status . is_server_error ()) } _ => false , } } }
    };
}

impl_70!();