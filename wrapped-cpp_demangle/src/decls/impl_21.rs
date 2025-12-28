macro_rules! deps {
    () => {
        AutoParseRecursion!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < 'a > Drop for AutoParseRecursion < 'a > { # [inline] fn drop (& mut self) { self . 0 . exit_recursion () ; } }
    };
}

impl_21!();