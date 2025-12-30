// Generated macro for arith_impl (module)
macro_rules! Depcratearith_impl {
() => {
// Module: crate
// Provides: {"arith_impl"}
// Dependencies: {}
# [cfg (feature = "derive")] mod arith_impl { # [doc = " Computes `a - (b + borrow)`, returning the result and the new borrow."] # [inline (always)] pub const fn sbb (a : u64 , b : u64 , borrow : u64) -> (u64 , u64) { let ret = (a as u128) . wrapping_sub ((b as u128) + ((borrow >> 63) as u128)) ; (ret as u64 , (ret >> 64) as u64) } # [doc = " Computes `a + b + carry`, returning the result and the new carry over."] # [inline (always)] pub const fn adc (a : u64 , b : u64 , carry : u64) -> (u64 , u64) { let ret = (a as u128) + (b as u128) + (carry as u128) ; (ret as u64 , (ret >> 64) as u64) } # [doc = " Computes `a + (b * c) + carry`, returning the result and the new carry over."] # [inline (always)] pub const fn mac (a : u64 , b : u64 , c : u64 , carry : u64) -> (u64 , u64) { let ret = (a as u128) + ((b as u128) * (c as u128)) + (carry as u128) ; (ret as u64 , (ret >> 64) as u64) } }
};
}
