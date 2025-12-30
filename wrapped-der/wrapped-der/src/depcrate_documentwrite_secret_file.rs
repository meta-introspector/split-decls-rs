// Generated macro for write_secret_file (function)
macro_rules! Depcrate_documentwrite_secret_file {
() => {
// Module: crate::document
// Provides: {"write_secret_file"}
// Dependencies: {}
# [doc = " Write a file containing secret data to the filesystem"] # [cfg (all (not (unix) , feature = "std" , feature = "zeroize"))] fn write_secret_file (path : impl AsRef < Path > , data : & [u8]) -> Result < () , Error > { fs :: write (path , data) ? ; Ok (()) }
};
}
