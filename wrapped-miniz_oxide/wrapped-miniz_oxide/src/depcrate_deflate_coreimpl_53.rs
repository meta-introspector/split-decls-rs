// Generated macro for impl_53 (impl)
macro_rules! Depcrate_deflate_coreimpl_53 {
() => {
// Module: crate::deflate::core
// Provides: {"impl_53"}
// Dependencies: {}
impl TDEFLFlush { pub const fn new (flush : i32) -> Result < Self , MZError > { match flush { 0 => Ok (TDEFLFlush :: None) , 1 => Ok (TDEFLFlush :: Partial) , 2 => Ok (TDEFLFlush :: Sync) , 3 => Ok (TDEFLFlush :: Full) , 4 => Ok (TDEFLFlush :: Finish) , _ => Err (MZError :: Param) , } } }
};
}
