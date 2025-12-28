macro_rules! deps {
    () => {
        MinInt!();
        HInt!();
    };
}

macro_rules! impl_h_int {
    () => {
        deps!();
        macro_rules ! impl_h_int { ($ ($ H : ident $ uH : ident $ X : ident) ,*) => { $ (impl HInt for $ H { type D = $ X ; fn widen (self) -> Self :: D { self as $ X } fn zero_widen (self) -> Self :: D { (self as $ uH) as $ X } fn zero_widen_mul (self , rhs : Self) -> Self :: D { self . zero_widen () . wrapping_mul (rhs . zero_widen ()) } fn widen_mul (self , rhs : Self) -> Self :: D { self . widen () . wrapping_mul (rhs . widen ()) } fn widen_hi (self) -> Self :: D { (self as $ X) << < Self as MinInt >:: BITS } }) * } ; }
    };
}

impl_h_int!();