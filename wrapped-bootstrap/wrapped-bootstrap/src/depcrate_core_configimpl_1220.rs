// Generated macro for impl_1220 (impl)
macro_rules! Depcrate_core_configimpl_1220 {
() => {
// Module: crate::core::config
// Provides: {"impl_1220"}
// Dependencies: {}
impl std :: str :: FromStr for RustcLto { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "thin-local" => Ok (RustcLto :: ThinLocal) , "thin" => Ok (RustcLto :: Thin) , "fat" => Ok (RustcLto :: Fat) , "off" => Ok (RustcLto :: Off) , _ => Err (format ! ("Invalid value for rustc LTO: {s}")) , } } }
};
}
