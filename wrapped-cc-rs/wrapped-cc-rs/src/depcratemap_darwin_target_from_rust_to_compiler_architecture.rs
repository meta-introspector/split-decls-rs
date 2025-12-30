// Generated macro for map_darwin_target_from_rust_to_compiler_architecture (function)
macro_rules! Depcratemap_darwin_target_from_rust_to_compiler_architecture {
() => {
// Module: crate
// Provides: {"map_darwin_target_from_rust_to_compiler_architecture"}
// Dependencies: {}
fn map_darwin_target_from_rust_to_compiler_architecture < 'a > (target : & TargetInfo < 'a >) -> & 'a str { match target . full_arch { "aarch64" => "arm64" , "arm64_32" => "arm64_32" , "arm64e" => "arm64e" , "armv7k" => "armv7k" , "armv7s" => "armv7s" , "i386" => "i386" , "i686" => "i386" , "powerpc" => "ppc" , "powerpc64" => "ppc64" , "x86_64" => "x86_64" , "x86_64h" => "x86_64h" , arch => arch , } }
};
}
