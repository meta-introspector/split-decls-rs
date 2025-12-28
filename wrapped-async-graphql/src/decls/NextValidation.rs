macro_rules! deps {
    () => {
        ValidationFut!();
        Extension!();
    };
}

macro_rules! NextValidation {
    () => {
        deps!();
        # [doc = " The remainder of a extension chain for validation."] pub struct NextValidation < 'a > { chain : & 'a [Arc < dyn Extension >] , validation_fut : ValidationFut < 'a > , }
    };
}

NextValidation!();