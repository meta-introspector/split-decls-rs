macro_rules! ApplyLeniencyDefaultValue {
    () => {
        pub trait ApplyLeniencyDefaultValue < T > { fn with_lenient_default_value (self , is_lenient : bool , default : T) -> Self ; }
    };
}

ApplyLeniencyDefaultValue!()