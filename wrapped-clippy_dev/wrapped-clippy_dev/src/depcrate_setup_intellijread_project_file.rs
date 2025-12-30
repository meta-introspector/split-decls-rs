// Generated macro for read_project_file (function)
macro_rules! Depcrate_setup_intellijread_project_file {
() => {
// Module: crate::setup::intellij
// Provides: {"read_project_file"}
// Dependencies: {}
# [doc = " `clippy_dev` expects to be executed in the root directory of Clippy. This function"] # [doc = " loads the given file or returns an error. Having it in this extra function ensures"] # [doc = " that the error message looks nice."] fn read_project_file (file_path : & str) -> Result < String , () > { let path = Path :: new (file_path) ; if ! path . exists () { eprintln ! ("error: unable to find the file `{file_path}`") ; return Err (()) ; } match fs :: read_to_string (path) { Ok (content) => Ok (content) , Err (err) => { eprintln ! ("error: the file `{file_path}` could not be read ({err})") ; Err (()) } , } }
};
}
