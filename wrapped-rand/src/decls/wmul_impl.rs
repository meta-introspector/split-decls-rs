macro_rules! deps {
    () => {
        WideningMultiply!();
    };
}

macro_rules! wmul_impl {
    () => {
        deps!();
        macro_rules ! wmul_impl { ($ ty : ty , $ wide : ty , $ shift : expr) => { impl WideningMultiply for $ ty { type Output = ($ ty , $ ty) ; # [inline (always)] fn wmul (self , x : $ ty) -> Self :: Output { let tmp = (self as $ wide) * (x as $ wide) ; ((tmp >> $ shift) as $ ty , tmp as $ ty) } } } ; ($ (($ ty : ident , $ wide : ty) ,) +, $ shift : expr) => { $ (impl WideningMultiply for $ ty { type Output = ($ ty , $ ty) ; # [inline (always)] fn wmul (self , x : $ ty) -> Self :: Output { let y : $ wide = self . cast () ; let x : $ wide = x . cast () ; let tmp = y * x ; let hi : $ ty = (tmp >> Simd :: splat ($ shift)) . cast () ; let lo : $ ty = tmp . cast () ; (hi , lo) } }) + } ; }
    };
}

wmul_impl!()