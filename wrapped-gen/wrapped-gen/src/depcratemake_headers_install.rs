// Generated macro for make_headers_install (function)
macro_rules! Depcratemake_headers_install {
() => {
// Module: crate
// Provides: {"make_headers_install"}
// Dependencies: {}
fn make_headers_install (linux_arch : & str , linux_headers : & Path) { assert ! (Command :: new ("make") . arg (format ! ("headers_install")) . arg (format ! ("ARCH={}" , linux_arch)) . arg (format ! ("INSTALL_HDR_PATH={}" , fs :: canonicalize (& linux_headers) . unwrap () . to_str () . unwrap ())) . current_dir ("linux") . status () . unwrap () . success ()) ; if linux_arch == "arm64" { fs :: copy ("linux/arch/arm64/include/asm/image.h" , linux_headers . join ("include/asm/image.h") ,) . expect ("Missing headers") ; } }
};
}
