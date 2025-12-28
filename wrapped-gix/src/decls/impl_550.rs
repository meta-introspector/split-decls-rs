macro_rules! deps {
    () => {
        ApplyLeniency!();
    };
}

macro_rules! impl_550 {
    () => {
        deps!();
        impl < T , E > ApplyLeniency for Result < Option < T > , E > { fn with_leniency (self , is_lenient : bool) -> Self { match self { Ok (v) => Ok (v) , Err (_) if is_lenient => Ok (None) , Err (err) => Err (err) , } } }
    };
}

impl_550!();