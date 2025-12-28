macro_rules! deps {
    () => {
        WideningMultiply!();
    };
}

macro_rules! wmul_impl_large {
    () => {
        deps!();
        macro_rules ! wmul_impl_large { ($ ty : ty , $ half : expr) => { impl WideningMultiply for $ ty { type Output = ($ ty , $ ty) ; # [inline (always)] fn wmul (self , b : $ ty) -> Self :: Output { const LOWER_MASK : $ ty = ! 0 >> $ half ; let mut low = (self & LOWER_MASK) . wrapping_mul (b & LOWER_MASK) ; let mut t = low >> $ half ; low &= LOWER_MASK ; t += (self >> $ half) . wrapping_mul (b & LOWER_MASK) ; low += (t & LOWER_MASK) << $ half ; let mut high = t >> $ half ; t = low >> $ half ; low &= LOWER_MASK ; t += (b >> $ half) . wrapping_mul (self & LOWER_MASK) ; low += (t & LOWER_MASK) << $ half ; high += t >> $ half ; high += (self >> $ half) . wrapping_mul (b >> $ half) ; (high , low) } } } ; (($ ($ ty : ty ,) +) $ scalar : ty , $ half : expr) => { $ (impl WideningMultiply for $ ty { type Output = ($ ty , $ ty) ; # [inline (always)] fn wmul (self , b : $ ty) -> Self :: Output { let lower_mask = <$ ty >:: splat (! 0 >> $ half) ; let half = <$ ty >:: splat ($ half) ; let mut low = (self & lower_mask) * (b & lower_mask) ; let mut t = low >> half ; low &= lower_mask ; t += (self >> half) * (b & lower_mask) ; low += (t & lower_mask) << half ; let mut high = t >> half ; t = low >> half ; low &= lower_mask ; t += (b >> half) * (self & lower_mask) ; low += (t & lower_mask) << half ; high += t >> half ; high += (self >> half) * (b >> half) ; (high , low) } }) + } ; }
    };
}

wmul_impl_large!();