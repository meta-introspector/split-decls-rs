macro_rules! WideningMultiply {
    () => {
        pub (crate) trait WideningMultiply < RHS = Self > { type Output ; fn wmul (self , x : RHS) -> Self :: Output ; }
    };
}

WideningMultiply!();