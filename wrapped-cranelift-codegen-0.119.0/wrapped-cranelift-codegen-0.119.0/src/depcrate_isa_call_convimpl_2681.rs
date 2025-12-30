// Generated macro for impl_2681 (impl)
macro_rules! Depcrate_isa_call_convimpl_2681 {
() => {
// Module: crate::isa::call_conv
// Provides: {"impl_2681"}
// Dependencies: {}
impl str :: FromStr for CallConv { type Err = () ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "fast" => Ok (Self :: Fast) , "cold" => Ok (Self :: Cold) , "tail" => Ok (Self :: Tail) , "system_v" => Ok (Self :: SystemV) , "windows_fastcall" => Ok (Self :: WindowsFastcall) , "apple_aarch64" => Ok (Self :: AppleAarch64) , "probestack" => Ok (Self :: Probestack) , "winch" => Ok (Self :: Winch) , _ => Err (()) , } } }
};
}
