// Generated macro for impl_5 (impl)
macro_rules! Depcrate_find_toolsimpl_5 {
() => {
// Module: crate::find_tools
// Provides: {"impl_5"}
// Dependencies: {}
impl TargetArch { # [doc = " Parse the `TargetArch` from a str. Returns `None` if the arch is unrecognized."] fn new (arch : & str) -> Option < Self > { match arch { "x64" | "x86_64" => Some (Self :: X64) , "arm64" | "aarch64" => Some (Self :: Arm64) , "arm64ec" => Some (Self :: Arm64ec) , "x86" | "i686" | "i586" => Some (Self :: X86) , "arm" | "thumbv7a" => Some (Self :: Arm) , _ => None , } } # [cfg (windows)] # [doc = " Gets the Visual Studio name for the architecture."] fn as_vs_arch (& self) -> & 'static str { match self { Self :: X64 => "x64" , Self :: Arm64 | Self :: Arm64ec => "arm64" , Self :: X86 => "x86" , Self :: Arm => "arm" , } } }
};
}
