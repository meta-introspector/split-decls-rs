// Generated macro for Cli (struct)
macro_rules! Depcrate_common_cliCli {
() => {
// Module: crate::common::cli
// Provides: {"Cli"}
// Dependencies: {}
# [doc = " Intrinsic test tool"] # [derive (clap :: Parser)] # [command (name = "Intrinsic test tool" , about = "Generates Rust and C programs for intrinsics and compares the output")] pub struct Cli { # [doc = " The input file containing the intrinsics"] pub input : PathBuf , # [doc = " The rust toolchain to use for building the rust code"] # [arg (long)] pub toolchain : Option < String > , # [doc = " The C++ compiler to use for compiling the c++ code"] # [arg (long , default_value_t = String :: from ("clang++"))] pub cppcompiler : String , # [doc = " Run the C programs under emulation with this command"] # [arg (long)] pub runner : Option < String > , # [doc = " Filename for a list of intrinsics to skip (one per line)"] # [arg (long)] pub skip : Option < PathBuf > , # [doc = " Regenerate test programs, but don't build or run them"] # [arg (long)] pub generate_only : bool , # [doc = " Pass a target the test suite"] # [arg (long , default_value_t = String :: from ("armv7-unknown-linux-gnueabihf"))] pub target : String , # [doc = " Set the linker"] # [arg (long)] pub linker : Option < String > , # [doc = " Set the sysroot for the C++ compiler"] # [arg (long)] pub cxx_toolchain_dir : Option < String > , # [arg (long , default_value_t = 100u8)] pub sample_percentage : u8 , }
};
}
