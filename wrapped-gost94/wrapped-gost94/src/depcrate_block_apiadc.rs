// Generated macro for adc (function)
macro_rules! Depcrate_block_apiadc {
() => {
// Module: crate::block_api
// Provides: {"adc"}
// Dependencies: {}
# [inline (always)] fn adc (a : & mut u64 , b : u64 , carry : & mut u64) { let ret = (* a as u128) + (b as u128) + (* carry as u128) ; * a = ret as u64 ; * carry = (ret >> 64) as u64 ; }
};
}
