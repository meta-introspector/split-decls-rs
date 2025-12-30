// Generated macro for impl_16 (impl)
macro_rules! Depcrate_targetimpl_16 {
() => {
// Module: crate::target
// Provides: {"impl_16"}
// Dependencies: {}
impl std :: str :: FromStr for Target { type Err = UnsupportedTarget ; fn from_str (s : & str) -> Result < Target , UnsupportedTarget > { match s { "nvptx64-nvidia-cuda" => Ok (Target :: Nvptx64NvidiaCuda) , _ => Err (UnsupportedTarget) , } } }
};
}
