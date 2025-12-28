macro_rules! ScalarValidatorFn {
    () => {
        # [doc = " A validator for scalar"] pub type ScalarValidatorFn = Arc < dyn Fn (& Value) -> bool + Send + Sync > ;
    };
}

ScalarValidatorFn!()