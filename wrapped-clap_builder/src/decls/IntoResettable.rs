macro_rules! deps {
    () => {
        Resettable!();
    };
}

macro_rules! IntoResettable {
    () => {
        deps!();
        # [doc = " Convert to the intended resettable type"] pub trait IntoResettable < T > { # [doc = " Convert to the intended resettable type"] fn into_resettable (self) -> Resettable < T > ; }
    };
}

IntoResettable!();