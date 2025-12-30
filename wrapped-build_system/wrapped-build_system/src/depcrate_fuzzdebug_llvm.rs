// Generated macro for debug_llvm (function)
macro_rules! Depcrate_fuzzdebug_llvm {
() => {
// Module: crate::fuzz
// Provides: {"debug_llvm"}
// Dependencies: {}
# [doc = " Builds & runs a file with LLVM."] fn debug_llvm (path : & std :: path :: Path) -> Result < Vec < u8 > , String > { let exe_path = path . with_extension ("llvm_elf") ; let output = std :: process :: Command :: new ("rustc") . arg (path) . arg ("-o") . arg (& exe_path) . output () . map_err (| err | format ! ("{err:?}")) ? ; if ! output . status . success () { return Err (format ! ("LLVM compilation failed:{output:?}")) ; } let output = std :: process :: Command :: new (& exe_path) . output () . map_err (| err | format ! ("{err:?}")) ? ; if ! output . status . success () { return Err (format ! ("The program at {path:?}, compiled with LLVM, exited unsuccessfully:{output:?}")) ; } std :: fs :: remove_file (exe_path) . map_err (| err | format ! ("{err:?}")) ? ; let mut res = output . stdout ; res . extend (output . stderr) ; Ok (res) }
};
}
