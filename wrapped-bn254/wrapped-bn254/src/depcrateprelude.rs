// Generated macro for prelude (module)
macro_rules! Depcrateprelude {
() => {
// Module: crate
// Provides: {"prelude"}
// Dependencies: {}
# [doc = " This module should be used by Solana programs or other downstream projects."] pub mod prelude { # [allow (deprecated)] # [cfg (not (target_os = "solana"))] pub use crate :: multiplication :: alt_bn128_multiplication_128 ; # [allow (deprecated)] pub use crate :: { addition :: { alt_bn128_addition , ALT_BN128_ADD , ALT_BN128_ADDITION_INPUT_LEN , ALT_BN128_ADDITION_INPUT_SIZE , ALT_BN128_ADDITION_OUTPUT_LEN , ALT_BN128_ADDITION_OUTPUT_SIZE , ALT_BN128_SUB , } , multiplication :: { alt_bn128_multiplication , ALT_BN128_MUL , ALT_BN128_MULTIPLICATION_INPUT_LEN , ALT_BN128_MULTIPLICATION_INPUT_SIZE , ALT_BN128_MULTIPLICATION_OUTPUT_LEN , ALT_BN128_MULTIPLICATION_OUTPUT_SIZE , } , pairing :: { alt_bn128_pairing , ALT_BN128_PAIRING , ALT_BN128_PAIRING_ELEMENT_LEN , ALT_BN128_PAIRING_OUTPUT_LEN , } , } ; pub use crate :: { addition :: { alt_bn128_g1_addition_be , alt_bn128_g1_addition_le , ALT_BN128_G1_ADDITION_INPUT_SIZE , ALT_BN128_G1_ADDITION_OUTPUT_SIZE , ALT_BN128_G1_ADD_BE , ALT_BN128_G1_ADD_LE , ALT_BN128_G1_SUB_BE , ALT_BN128_G1_SUB_LE , } , consts :: * , multiplication :: { alt_bn128_g1_multiplication_be , alt_bn128_g1_multiplication_le , ALT_BN128_G1_MULTIPLICATION_INPUT_SIZE , ALT_BN128_G1_MULTIPLICATION_OUTPUT_SIZE , ALT_BN128_G1_MUL_BE , ALT_BN128_G1_MUL_LE , } , pairing :: { alt_bn128_pairing_be , alt_bn128_pairing_le , ALT_BN128_PAIRING_BE , ALT_BN128_PAIRING_ELEMENT_SIZE , ALT_BN128_PAIRING_LE , ALT_BN128_PAIRING_OUTPUT_SIZE , } , AltBn128Error , } ; }
};
}
