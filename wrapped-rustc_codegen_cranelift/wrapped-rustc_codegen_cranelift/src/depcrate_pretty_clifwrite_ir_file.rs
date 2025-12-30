// Generated macro for write_ir_file (function)
macro_rules! Depcrate_pretty_clifwrite_ir_file {
() => {
// Module: crate::pretty_clif
// Provides: {"write_ir_file"}
// Dependencies: {}
pub (crate) fn write_ir_file (output_filenames : & OutputFilenames , name : & str , write : impl FnOnce (& mut dyn Write) -> std :: io :: Result < () > ,) { let clif_output_dir = output_filenames . with_extension ("clif") ; match std :: fs :: create_dir (& clif_output_dir) { Ok (()) => { } Err (err) if err . kind () == std :: io :: ErrorKind :: AlreadyExists => { } res @ Err (_) => res . unwrap () , } let clif_file_name = clif_output_dir . join (name) ; let res = std :: fs :: File :: create (clif_file_name) . and_then (| mut file | write (& mut file)) ; if let Err (err) = res { let handler = rustc_session :: EarlyDiagCtxt :: new (rustc_session :: config :: ErrorOutputType :: default ()) ; handler . early_warn (format ! ("error writing ir file: {}" , err)) ; } }
};
}
