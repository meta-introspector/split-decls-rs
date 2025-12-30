// Generated macro for Commands (enum)
macro_rules! DepcrateCommands {
() => {
// Module: crate
// Provides: {"Commands"}
// Dependencies: {}
# [derive (Subcommand)] enum Commands { # [doc = " Compress a single file. If no output file is specified,"] # [doc = " output will be written to <INPUT_FILE>.zst"] Compress { # [doc = " File to compress"] input_file : PathBuf , # [doc = " Where the compressed file is written"] # [doc = " [default: <INPUT_FILE>.zst]"] output_file : Option < PathBuf > , # [doc = " How thoroughly the file should be compressed. A higher level will take"] # [doc = " more time to compress but result in a smaller file, and vice versa."] # [doc = ""] # [doc = " - 0: Uncompressed"] # [doc = " - 1: Fastest"] # [doc = " - 2: Default"] # [doc = " - 3: Better"] # [doc = " - 4: Best"] # [arg (short , long , value_name = "COMPRESSION_LEVEL" , default_value_t = 2 , verbatim_doc_comment)] level : u8 , } , Decompress { # [doc = " .zst archive to decompress"] input_file : PathBuf , # [doc = " Where the compressed file is written"] # [doc = " [default: <ARCHIVE_NAME>]"] output_file : Option < PathBuf > , } , }
};
}
