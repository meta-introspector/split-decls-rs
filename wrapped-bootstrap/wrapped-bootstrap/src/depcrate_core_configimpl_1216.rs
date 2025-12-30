// Generated macro for impl_1216 (impl)
macro_rules! Depcrate_core_configimpl_1216 {
() => {
// Module: crate::core::config
// Provides: {"impl_1216"}
// Dependencies: {}
impl std :: str :: FromStr for SplitDebuginfo { type Err = () ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "packed" => Ok (SplitDebuginfo :: Packed) , "unpacked" => Ok (SplitDebuginfo :: Unpacked) , "off" => Ok (SplitDebuginfo :: Off) , _ => Err (()) , } } }
};
}
