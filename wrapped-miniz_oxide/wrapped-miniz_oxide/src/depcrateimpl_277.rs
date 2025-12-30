// Generated macro for impl_277 (impl)
macro_rules! Depcrateimpl_277 {
() => {
// Module: crate
// Provides: {"impl_277"}
// Dependencies: {}
impl MZFlush { # [doc = " Create an MZFlush value from an integer value."] # [doc = ""] # [doc = " Returns `MZError::Param` on invalid values."] pub fn new (flush : i32) -> Result < Self , MZError > { match flush { 0 => Ok (MZFlush :: None) , 1 | 2 => Ok (MZFlush :: Sync) , 3 => Ok (MZFlush :: Full) , 4 => Ok (MZFlush :: Finish) , _ => Err (MZError :: Param) , } } }
};
}
