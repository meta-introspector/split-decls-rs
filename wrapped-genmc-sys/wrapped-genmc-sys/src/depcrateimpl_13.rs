// Generated macro for impl_13 (impl)
macro_rules! Depcrateimpl_13 {
() => {
// Module: crate
// Provides: {"impl_13"}
// Dependencies: {}
impl FromStr for LogLevel { type Err = & 'static str ; fn from_str (s : & str) -> Result < Self , Self :: Err > { Ok (match s { "quiet" => LogLevel :: Quiet , "error" => LogLevel :: Error , "warning" => LogLevel :: Warning , "tip" => LogLevel :: Tip , "debug1" => LogLevel :: Debug1Revisits , "debug2" => LogLevel :: Debug2MemoryAccesses , "debug3" => LogLevel :: Debug3ReadsFrom , _ => return Err ("invalid log level") , }) } }
};
}
