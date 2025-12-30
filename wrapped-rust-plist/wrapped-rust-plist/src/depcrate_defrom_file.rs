// Generated macro for from_file (function)
macro_rules! Depcrate_defrom_file {
() => {
// Module: crate::de
// Provides: {"from_file"}
// Dependencies: {}
# [doc = " Deserializes an instance of type `T` from a plist file of any encoding."] pub fn from_file < P : AsRef < Path > , T : de :: DeserializeOwned > (path : P) -> Result < T , Error > { let file = File :: open (path) . map_err (error :: from_io_without_position) ? ; from_reader (BufReader :: new (file)) }
};
}
