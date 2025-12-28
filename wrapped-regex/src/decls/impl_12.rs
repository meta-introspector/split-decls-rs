macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for Error { # [allow (deprecated)] fn description (& self) -> & str { match * self { Error :: Syntax (ref err) => err , Error :: CompiledTooBig (_) => "compiled program too big" , } } }
    };
}

impl_12!()