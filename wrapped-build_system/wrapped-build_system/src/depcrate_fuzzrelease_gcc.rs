// Generated macro for release_gcc (function)
macro_rules! Depcrate_fuzzrelease_gcc {
() => {
// Module: crate::fuzz
// Provides: {"release_gcc"}
// Dependencies: {}
# [doc = " Builds & runs a file with GCC."] fn release_gcc (path : & std :: path :: Path) -> Result < Vec < u8 > , String > { let exe_path = path . with_extension ("gcc_elf") ; let output = std :: process :: Command :: new ("./y.sh") . arg ("rustc") . arg (path) . arg ("-O") . arg ("-o") . arg (& exe_path) . output () . map_err (| err | format ! ("{err:?}")) ? ; if ! output . status . success () { return Err (format ! ("GCC compilation failed:{output:?}")) ; } let output = std :: process :: Command :: new (& exe_path) . output () . map_err (| err | format ! ("{err:?}")) ? ; if ! output . status . success () { return Err (format ! ("The program at {path:?}, compiled with GCC, exited unsuccessfully:{output:?}")) ; } std :: fs :: remove_file (exe_path) . map_err (| err | format ! ("{err:?}")) ? ; let mut res = output . stdout ; res . extend (output . stderr) ; Ok (res) }
};
}
