// Generated macro for versioned (module)
macro_rules! Depcrateversioned {
() => {
// Module: crate
// Provides: {"versioned"}
// Dependencies: {}
# [doc = " This module contains the versioned syscall implementations and is intended for use"] # [doc = " primarily by validator code."] # [cfg (not (target_os = "solana"))] pub mod versioned { pub use crate :: { addition :: { alt_bn128_versioned_g1_addition , VersionedG1Addition , ALT_BN128_G1_ADDITION_INPUT_SIZE , ALT_BN128_G1_ADDITION_OUTPUT_SIZE , ALT_BN128_G1_ADD_BE , ALT_BN128_G1_ADD_LE , ALT_BN128_G1_SUB_BE , ALT_BN128_G1_SUB_LE , } , multiplication :: { alt_bn128_versioned_g1_multiplication , VersionedG1Multiplication , ALT_BN128_G1_MULTIPLICATION_INPUT_SIZE , ALT_BN128_G1_MULTIPLICATION_OUTPUT_SIZE , ALT_BN128_G1_MUL_BE , ALT_BN128_G1_MUL_LE , } , pairing :: { alt_bn128_versioned_pairing , VersionedPairing , ALT_BN128_PAIRING_BE , ALT_BN128_PAIRING_ELEMENT_SIZE , ALT_BN128_PAIRING_LE , ALT_BN128_PAIRING_OUTPUT_SIZE , } , target_arch :: Endianness , } ; # [allow (deprecated)] pub use crate :: { addition :: { ALT_BN128_ADD , ALT_BN128_ADDITION_INPUT_LEN , ALT_BN128_ADDITION_INPUT_SIZE , ALT_BN128_ADDITION_OUTPUT_LEN , ALT_BN128_ADDITION_OUTPUT_SIZE , ALT_BN128_SUB , } , multiplication :: { ALT_BN128_MUL , ALT_BN128_MULTIPLICATION_INPUT_LEN , ALT_BN128_MULTIPLICATION_INPUT_SIZE , ALT_BN128_MULTIPLICATION_OUTPUT_LEN , ALT_BN128_MULTIPLICATION_OUTPUT_SIZE , } , pairing :: { ALT_BN128_PAIRING , ALT_BN128_PAIRING_ELEMENT_LEN , ALT_BN128_PAIRING_OUTPUT_LEN } , } ; }
};
}
