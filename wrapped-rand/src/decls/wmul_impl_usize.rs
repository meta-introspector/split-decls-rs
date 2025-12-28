macro_rules! deps {
    () => {
        WideningMultiply!();
    };
}

macro_rules! wmul_impl_usize {
    () => {
        deps!();
        macro_rules ! wmul_impl_usize { ($ ty : ty) => { impl WideningMultiply for usize { type Output = (usize , usize) ; # [inline (always)] fn wmul (self , x : usize) -> Self :: Output { let (high , low) = (self as $ ty) . wmul (x as $ ty) ; (high as usize , low as usize) } } } ; }
    };
}

wmul_impl_usize!();