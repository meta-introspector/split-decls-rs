// Generated macro for read_to_string (function)
macro_rules! Depcrate_fileread_to_string {
() => {
// Module: crate::file
// Provides: {"read_to_string"}
// Dependencies: {}
# [doc = " Read file contents, placing them into `String`."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return an error in the following situations, but is not limited to just"] # [doc = " these cases:"] # [doc = ""] # [doc = " * This `path` is not a file."] # [doc = " * This `path` file does not exist."] # [doc = " * The current process does not have the permission to access `path`."] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust,ignore"] # [doc = " extern crate fs_extra;"] # [doc = " use fs_extra::file::read_to_string;"] # [doc = ""] # [doc = " let file_content = read_to_string(\"foo.txt\" )?; // Get file content from foo.txt"] # [doc = " println!(\"{}\", file_content);"] # [doc = ""] # [doc = " ```"] pub fn read_to_string < P > (path : P) -> Result < String > where P : AsRef < Path > , { let path = path . as_ref () ; if path . exists () && ! path . is_file () { if let Some (msg) = path . to_str () { let msg = format ! ("Path \"{}\" is not a file!" , msg) ; err ! (& msg , ErrorKind :: InvalidFile) ; } err ! ("Path is not a file!" , ErrorKind :: InvalidFile) ; } let mut file = File :: open (path) ? ; let mut result = String :: new () ; file . read_to_string (& mut result) ? ; Ok (result) }
};
}
