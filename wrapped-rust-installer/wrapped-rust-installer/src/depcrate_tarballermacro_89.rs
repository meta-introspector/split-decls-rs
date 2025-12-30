// Generated macro for macro_89 (macro)
macro_rules! Depcrate_tarballermacro_89 {
() => {
// Module: crate::tarballer
// Provides: {"macro_89"}
// Dependencies: {}
actor ! { # [derive (Debug)] pub struct Tarballer { # [doc = " The input folder to be compressed."] # [arg (value_name = "NAME")] input : String = "package" , # [doc = " The prefix of the tarballs."] # [arg (value_name = "PATH")] output : String = "./dist" , # [doc = " The folder in which the input is to be found."] # [arg (value_name = "DIR")] work_dir : String = "./workdir" , # [doc = " The profile used to compress the tarball."] # [arg (value_name = "FORMAT" , default_value_t)] compression_profile : CompressionProfile , # [doc = " The formats used to compress the tarball."] # [arg (value_name = "FORMAT" , default_value_t)] compression_formats : CompressionFormats , # [doc = " Modification time that will be set for all files added to the archive."] # [doc = " The default is the date of the first Rust commit from 2006."] # [doc = " This serves for better reproducibility of the archives."] # [arg (value_name = "FILE_MTIME" , default_value_t = 1153704088)] override_file_mtime : u64 , } }
};
}
