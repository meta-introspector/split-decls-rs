// Generated macro for save_in (function)
macro_rules! Depcrate_persist_file_formatsave_in {
() => {
// Module: crate::persist::file_format
// Provides: {"save_in"}
// Dependencies: {}
pub (crate) fn save_in < F > (sess : & Session , path_buf : PathBuf , name : & str , encode : F) where F : FnOnce (FileEncoder) -> FileEncodeResult , { debug ! ("save: storing data in {}" , path_buf . display ()) ; match fs :: remove_file (& path_buf) { Ok (()) => { debug ! ("save: remove old file") ; } Err (err) if err . kind () == io :: ErrorKind :: NotFound => () , Err (err) => sess . dcx () . emit_fatal (errors :: DeleteOld { name , path : path_buf , err }) , } let mut encoder = match FileEncoder :: new (& path_buf) { Ok (encoder) => encoder , Err (err) => sess . dcx () . emit_fatal (errors :: CreateNew { name , path : path_buf , err }) , } ; write_file_header (& mut encoder , sess) ; match encode (encoder) { Ok (position) => { sess . prof . artifact_size (& name . replace (' ' , "_") , path_buf . file_name () . unwrap () . to_string_lossy () , position as u64 ,) ; debug ! ("save: data written to disk successfully") ; } Err ((path , err)) => sess . dcx () . emit_fatal (errors :: WriteNew { name , path , err }) , } }
};
}
