macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl Error { pub (crate) fn from_meta_build_error (err : meta :: BuildError) -> Error { if let Some (size_limit) = err . size_limit () { Error :: CompiledTooBig (size_limit) } else if let Some (ref err) = err . syntax_error () { Error :: Syntax (err . to_string ()) } else { Error :: Syntax (err . to_string ()) } } }
    };
}

impl_11!();