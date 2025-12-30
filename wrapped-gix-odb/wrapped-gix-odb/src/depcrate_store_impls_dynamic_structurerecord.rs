// Generated macro for Record (enum)
macro_rules! Depcrate_store_impls_dynamic_structureRecord {
() => {
// Module: crate::store_impls::dynamic::structure
// Provides: {"Record"}
// Dependencies: {}
# [doc = " A record of a structural element of an object database."] # [derive (Debug , Clone , PartialEq , Eq)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub enum Record { # [doc = " A loose object database."] LooseObjectDatabase { # [doc = " The root of the object database."] objects_directory : PathBuf , # [doc = " The amount of object files."] num_objects : usize , } , # [doc = " A pack index file"] Index { # [doc = " The location of the index file,"] path : PathBuf , # [doc = " Whether or not the index is mapped into memory."] state : IndexState , } , # [doc = " A multi-index file"] MultiIndex { # [doc = " The location of the multi-index file,"] path : PathBuf , # [doc = " Whether or not the index is mapped into memory."] state : IndexState , } , # [doc = " An empty slot was encountered, this is possibly happening as the ODB changes during query with"] # [doc = " a file being removed."] Empty , }
};
}
