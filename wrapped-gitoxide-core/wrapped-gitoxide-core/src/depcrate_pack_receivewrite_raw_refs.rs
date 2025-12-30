// Generated macro for write_raw_refs (function)
macro_rules! Depcrate_pack_receivewrite_raw_refs {
() => {
// Module: crate::pack::receive
// Provides: {"write_raw_refs"}
// Dependencies: {}
fn write_raw_refs (refs : & [Ref] , directory : PathBuf) -> std :: io :: Result < () > { let assure_dir_exists = | path : & BString | { assert ! (! path . starts_with_str ("/") , "no ref start with a /, they are relative") ; let path = directory . join (gix :: path :: from_byte_slice (path)) ; std :: fs :: create_dir_all (path . parent () . expect ("multi-component path")) . map (| _ | path) } ; for r in refs { let (path , content) = match r { Ref :: Unborn { full_ref_name , target } => { (assure_dir_exists (full_ref_name) ? , format ! ("unborn HEAD: {target}")) } Ref :: Symbolic { full_ref_name : path , target , .. } => (assure_dir_exists (path) ? , format ! ("ref: {target}")) , Ref :: Peeled { full_ref_name : path , tag : object , .. } | Ref :: Direct { full_ref_name : path , object , } => (assure_dir_exists (path) ? , object . to_string ()) , } ; std :: fs :: write (path , content . as_bytes ()) ? ; } Ok (()) }
};
}
