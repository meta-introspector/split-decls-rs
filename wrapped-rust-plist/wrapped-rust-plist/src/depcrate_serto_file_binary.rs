// Generated macro for to_file_binary (function)
macro_rules! Depcrate_serto_file_binary {
() => {
// Module: crate::ser
// Provides: {"to_file_binary"}
// Dependencies: {}
# [doc = " Serializes the given data structure to a file as a binary encoded plist."] pub fn to_file_binary < P : AsRef < Path > , T : ser :: Serialize > (path : P , value : & T) -> Result < () , Error > { let mut file = File :: create (path) . map_err (error :: from_io_without_position) ? ; to_writer_binary (BufWriter :: new (& mut file) , value) ? ; file . sync_all () . map_err (error :: from_io_without_position) ? ; Ok (()) }
};
}
