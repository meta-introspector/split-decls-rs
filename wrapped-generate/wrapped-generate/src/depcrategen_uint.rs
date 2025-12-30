// Generated macro for gen_uint (function)
macro_rules! Depcrategen_uint {
() => {
// Module: crate
// Provides: {"gen_uint"}
// Dependencies: {}
fn gen_uint (u : u64) -> UIntCode { let mut result = UIntCode :: Term ; let mut x = 1u64 << 63 ; while x > u { x >>= 1 } while x > 0 { result = if x & u > 0 { UIntCode :: One (Box :: new (result)) } else { UIntCode :: Zero (Box :: new (result)) } ; x >>= 1 ; } result }
};
}
