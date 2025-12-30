// Generated macro for DecompressionCommand (struct)
macro_rules! Depcrate_decompressDecompressionCommand {
() => {
// Module: crate::decompress
// Provides: {"DecompressionCommand"}
// Dependencies: {}
# [doc = " A representation of a single command for decompressing data"] # [doc = " out-of-process."] # [derive (Clone , Debug)] struct DecompressionCommand { # [doc = " The glob that matches this command."] glob : String , # [doc = " The command or binary name."] bin : PathBuf , # [doc = " The arguments to invoke with the command."] args : Vec < OsString > , }
};
}
