macro_rules! ApplyLeniency {
    () => {
        pub trait ApplyLeniency { fn with_leniency (self , is_lenient : bool) -> Self ; }
    };
}

ApplyLeniency!()