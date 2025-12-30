// Generated macro for Key (struct)
macro_rules! Depcrate_cmacKey {
() => {
// Module: crate::cmac
// Provides: {"Key"}
// Dependencies: {}
# [doc = " A key to use for CMAC signing."] # [derive (Clone)] pub struct Key { algorithm : Algorithm , ctx : LcPtr < CMAC_CTX > , }
};
}
