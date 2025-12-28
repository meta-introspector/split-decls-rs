macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! ensure_compatible_types {
    () => {
        deps!();
        # [inline] pub (crate) fn ensure_compatible_types < T , E > () -> Result < () , Error > { if size_of :: < T > () != size_of :: < E > () { Err (Error :: IncompatibleSize) } else { Ok (()) } }
    };
}

ensure_compatible_types!()