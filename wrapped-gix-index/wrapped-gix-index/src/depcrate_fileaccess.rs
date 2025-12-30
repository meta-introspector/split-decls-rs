// Generated macro for access (module)
macro_rules! Depcrate_fileaccess {
() => {
// Module: crate::file
// Provides: {"access"}
// Dependencies: {}
mod access { use crate :: File ; # [doc = " Consumption"] impl File { # [doc = " Take all non-copy parts of the index."] pub fn into_parts (self) -> (crate :: State , std :: path :: PathBuf) { (self . state , self . path) } } # [doc = " Access"] impl File { # [doc = " The path from which the index was read or to which it is supposed to be written when used with [`File::from_state()`]."] pub fn path (& self) -> & std :: path :: Path { & self . path } # [doc = " The checksum over the file that was read or written to disk, or `None` if the state in memory was never serialized."] # [doc = ""] # [doc = " Note that even if `Some`, it will only represent the state in memory right after reading or [writing][File::write()]."] pub fn checksum (& self) -> Option < gix_hash :: ObjectId > { self . checksum } } }
};
}
