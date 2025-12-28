macro_rules! deps {
    () => {
        UnwrapOrElseFn!();
    };
}

macro_rules! unwrap_or_else_fn {
    () => {
        deps!();
        pub (crate) fn unwrap_or_else_fn < F > (f : F) -> UnwrapOrElseFn < F > { UnwrapOrElseFn (f) }
    };
}

unwrap_or_else_fn!();