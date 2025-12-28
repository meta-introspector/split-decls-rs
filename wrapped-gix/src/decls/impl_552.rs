macro_rules! deps {
    () => {
        ApplyLeniencyDefaultValue!();
    };
}

macro_rules! impl_552 {
    () => {
        deps!();
        impl < T , E > ApplyLeniencyDefaultValue < T > for Result < T , E > { fn with_lenient_default_value (self , is_lenient : bool , default : T) -> Self { match self { Ok (v) => Ok (v) , Err (_) if is_lenient => Ok (default) , Err (err) => Err (err) , } } }
    };
}

impl_552!();