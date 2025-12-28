macro_rules! deps {
    () => {
        ArrayLength!();
    };
}

macro_rules! impl_159 {
    () => {
        deps!();
        unsafe impl ArrayLength for UTerm { # [doc (hidden)] type ArrayType < T > = [T ; 0] ; }
    };
}

impl_159!()