// Generated macro for to_writer (function)
macro_rules! Depcrate_ser_helpersto_writer {
() => {
// Module: crate::ser::helpers
// Provides: {"to_writer"}
// Dependencies: {}
# [doc = " Serializes an object directly into a `Writer`."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # #[cfg(feature = \"std\")]"] # [doc = " let stderr = std::io::stderr();"] # [doc = " # #[cfg(feature = \"std\")]"] # [doc = " assert_eq!((), borsh::to_writer(&stderr, \"hello_0x0a\").unwrap());"] # [doc = " ```"] pub fn to_writer < T , W : Write > (mut writer : W , value : & T) -> Result < () > where T : BorshSerialize + ? Sized , { value . serialize (& mut writer) }
};
}
