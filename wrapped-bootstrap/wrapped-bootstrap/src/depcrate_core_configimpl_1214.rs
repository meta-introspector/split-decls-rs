// Generated macro for impl_1214 (impl)
macro_rules! Depcrate_core_configimpl_1214 {
() => {
// Module: crate::core::config
// Provides: {"impl_1214"}
// Dependencies: {}
impl FromStr for LlvmLibunwind { type Err = String ; fn from_str (value : & str) -> Result < Self , Self :: Err > { match value { "no" => Ok (Self :: No) , "in-tree" => Ok (Self :: InTree) , "system" => Ok (Self :: System) , invalid => Err (format ! ("Invalid value '{invalid}' for rust.llvm-libunwind config.")) , } } }
};
}
