// Generated macro for is_download_ci_available (function)
macro_rules! Depcrate_core_downloadis_download_ci_available {
() => {
// Module: crate::core::download
// Provides: {"is_download_ci_available"}
// Dependencies: {}
# [doc = " Checks whether the CI rustc is available for the given target triple."] pub (crate) fn is_download_ci_available (target_triple : & str , llvm_assertions : bool) -> bool { const SUPPORTED_PLATFORMS : & [& str] = & ["aarch64-apple-darwin" , "aarch64-pc-windows-msvc" , "aarch64-unknown-linux-gnu" , "aarch64-unknown-linux-musl" , "arm-unknown-linux-gnueabi" , "arm-unknown-linux-gnueabihf" , "armv7-unknown-linux-gnueabihf" , "i686-pc-windows-gnu" , "i686-pc-windows-msvc" , "i686-unknown-linux-gnu" , "loongarch64-unknown-linux-gnu" , "powerpc-unknown-linux-gnu" , "powerpc64-unknown-linux-gnu" , "powerpc64le-unknown-linux-gnu" , "powerpc64le-unknown-linux-musl" , "riscv64gc-unknown-linux-gnu" , "s390x-unknown-linux-gnu" , "x86_64-apple-darwin" , "x86_64-pc-windows-gnu" , "x86_64-pc-windows-msvc" , "x86_64-unknown-freebsd" , "x86_64-unknown-illumos" , "x86_64-unknown-linux-gnu" , "x86_64-unknown-linux-musl" , "x86_64-unknown-netbsd" ,] ; const SUPPORTED_PLATFORMS_WITH_ASSERTIONS : & [& str] = & ["x86_64-unknown-linux-gnu" , "x86_64-pc-windows-msvc"] ; if llvm_assertions { SUPPORTED_PLATFORMS_WITH_ASSERTIONS . contains (& target_triple) } else { SUPPORTED_PLATFORMS . contains (& target_triple) } }
};
}
