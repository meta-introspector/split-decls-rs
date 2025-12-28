macro_rules! deps {
    () => {
        ApplyLeniencyDefault!();
        Default!();
    };
}

macro_rules! impl_551 {
    () => {
        deps!();
        impl < T , E > ApplyLeniencyDefault for Result < T , E > where T : Default , { fn with_lenient_default (self , is_lenient : bool) -> Self { match self { Ok (v) => Ok (v) , Err (_) if is_lenient => Ok (T :: default ()) , Err (err) => Err (err) , } } }
    };
}

impl_551!();