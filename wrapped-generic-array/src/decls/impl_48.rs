macro_rules! deps {
    () => {
        LengthError!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        # [rustversion :: since (1.81)] impl core :: error :: Error for LengthError { }
    };
}

impl_48!()