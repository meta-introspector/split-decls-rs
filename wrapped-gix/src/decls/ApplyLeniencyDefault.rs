macro_rules! ApplyLeniencyDefault {
    () => {
        pub trait ApplyLeniencyDefault { fn with_lenient_default (self , is_lenient : bool) -> Self ; }
    };
}

ApplyLeniencyDefault!();