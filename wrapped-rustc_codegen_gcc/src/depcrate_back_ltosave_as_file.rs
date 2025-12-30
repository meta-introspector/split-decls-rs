// Generated macro for save_as_file (function)
macro_rules! Depcrate_back_ltosave_as_file {
() => {
// Module: crate::back::lto
// Provides: {"save_as_file"}
// Dependencies: {}
fn save_as_file (obj : & [u8] , path : & Path) -> Result < () , LtoBitcodeFromRlib > { fs :: write (path , obj) . map_err (| error | LtoBitcodeFromRlib { gcc_err : format ! ("write object file to temp dir: {}" , error) , }) }
};
}
