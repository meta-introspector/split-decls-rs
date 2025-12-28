macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl :: std :: error :: Error for Error { fn description (& self) -> & str { match * self { Error :: InvalidChar => "invalid character" , Error :: InvalidLength (_) => "invalid length" , Error :: Overflow => "overflow" , } } }
    };
}

impl_56!();