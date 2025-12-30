// Generated macro for File (struct)
macro_rules! Depcrate_indexFile {
() => {
// Module: crate::index
// Provides: {"File"}
// Dependencies: {}
# [doc = " A representation of a pack index file"] pub struct File { data : Mmap , path : std :: path :: PathBuf , version : Version , num_objects : u32 , fan : [u32 ; FAN_LEN] , hash_len : usize , object_hash : gix_hash :: Kind , }
};
}
