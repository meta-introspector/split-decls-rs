// Generated macro for array_impls (macro)
macro_rules! Depcratearray_impls {
() => {
// Module: crate
// Provides: {"array_impls"}
// Dependencies: {}
macro_rules ! array_impls { ($ ($ len : expr) +) => { $ (impl TryFrom < [u8 ; $ len] > for ByteString { type Error = str :: Utf8Error ; # [inline] fn try_from (value : [u8 ; $ len]) -> Result < Self , Self :: Error > { ByteString :: try_from (& value [..]) } } impl TryFrom <& [u8 ; $ len] > for ByteString { type Error = str :: Utf8Error ; # [inline] fn try_from (value : & [u8 ; $ len]) -> Result < Self , Self :: Error > { ByteString :: try_from (& value [..]) } }) + } }
};
}
