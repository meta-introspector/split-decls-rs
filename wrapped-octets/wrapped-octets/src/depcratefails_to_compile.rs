// Generated macro for fails_to_compile (module)
macro_rules! Depcratefails_to_compile {
() => {
// Module: crate
// Provides: {"fails_to_compile"}
// Dependencies: {}
# [doc = " The functions in this mod test the compile time assertions in the"] # [doc = " `put_u` and `peek_u` macros. If you compile this crate with"] # [doc = " `--cfg test_invalid_len_compilation_fail`, e.g., by using"] # [doc = " `cargo rustc  -- --cfg test_invalid_len_compilation_fail`"] # [doc = " You will get two compiler errors"] # [cfg (test_invalid_len_compilation_fail)] pub mod fails_to_compile { use super :: * ; pub fn peek_invalid_fails_to_compile (b : & mut Octets) -> Result < u8 > { peek_u ! (b , u8 , 2) } pub fn put_invalid_fails_to_compile < 'a > (b : & 'a mut OctetsMut , v : u8 ,) -> Result < & 'a mut [u8] > { put_u ! (b , u8 , v , 2) } }
};
}
