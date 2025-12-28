macro_rules! deps {
    () => {
        OsError!();
        Error!();
    };
}

macro_rules! impl_264 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for OsError { # [inline] fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { std :: error :: Error :: source (& self . 0) } }
    };
}

impl_264!();