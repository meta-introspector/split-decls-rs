// Generated macro for to_file_xml (function)
macro_rules! Depcrate_serto_file_xml {
() => {
// Module: crate::ser
// Provides: {"to_file_xml"}
// Dependencies: {}
# [doc = " Serializes the given data structure to a file as an XML encoded plist."] pub fn to_file_xml < P : AsRef < Path > , T : ser :: Serialize > (path : P , value : & T) -> Result < () , Error > { let mut file = File :: create (path) . map_err (error :: from_io_without_position) ? ; to_writer_xml (BufWriter :: new (& mut file) , value) ? ; file . sync_all () . map_err (error :: from_io_without_position) ? ; Ok (()) }
};
}
