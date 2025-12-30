// Generated macro for File (struct)
macro_rules! Depcrate_dataFile {
() => {
// Module: crate::data
// Provides: {"File"}
// Dependencies: {}
# [doc = " A pack data file"] pub struct File { data : Mmap , path : std :: path :: PathBuf , # [doc = " A value to represent this pack uniquely when used with cache lookup, or a way to identify this pack by its location on disk."] # [doc = " The same location on disk should yield the same id."] # [doc = ""] # [doc = " These must be unique per pack and must be stable, that is they don't change if the pack doesn't change."] # [doc = " If the same id is assigned (or reassigned) to different packs, pack creation or cache access will fail in hard-to-debug ways."] # [doc = ""] # [doc = " This value is controlled by the owning object store, which can use it in whichever way it wants as long as the above constraints are met."] pub id : Id , version : Version , num_objects : u32 , # [doc = " The size of the hash contained within. This is entirely determined by the caller, and repositories have to know which hash to use"] # [doc = " based on their configuration."] hash_len : usize , object_hash : gix_hash :: Kind , }
};
}
