// Generated macro for Repo (struct)
macro_rules! Depcrate_corpus_dbRepo {
() => {
// Module: crate::corpus::db
// Provides: {"Repo"}
// Dependencies: {}
# [doc = " a husk of a repository"] pub (crate) struct Repo { pub (crate) id : Id , # [doc = " The full path to the repository on disk, not yet validated to exist."] pub (crate) path : PathBuf , # [doc = " The size of the object database, counted quickly by packs only."] pub (crate) odb_size : ByteSize , # [doc = " The amount of objects stored in the object database."] pub (crate) num_objects : u64 , # [doc = " The total amount of references, no matter which type."] pub (crate) num_references : usize , }
};
}
