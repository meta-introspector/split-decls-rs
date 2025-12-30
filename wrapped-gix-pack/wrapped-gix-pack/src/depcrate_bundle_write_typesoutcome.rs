// Generated macro for Outcome (struct)
macro_rules! Depcrate_bundle_write_typesOutcome {
() => {
// Module: crate::bundle::write::types
// Provides: {"Outcome"}
// Dependencies: {}
# [doc = " Returned by [`write_to_directory`][crate::Bundle::write_to_directory()] or"] # [doc = " [`write_to_directory_eagerly`][crate::Bundle::write_to_directory_eagerly()]"] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Outcome { # [doc = " The successful result of the index write operation."] pub index : crate :: index :: write :: Outcome , # [doc = " The version of the pack."] pub pack_version : crate :: data :: Version , # [doc = " The kind of hash stored within the pack and indices."] pub object_hash : gix_hash :: Kind , # [doc = " The path to the pack index file."] pub index_path : Option < PathBuf > , # [doc = " The path to the pack data file."] pub data_path : Option < PathBuf > , # [doc = " The path to the `.keep` file to prevent collection of the newly written pack until refs are pointing to it."] # [doc = " It might be `None` if the file at `data_path` already existed, indicating that we have received a pack that"] # [doc = " was already present locally."] # [doc = ""] # [doc = " The file is created right before moving the pack data and index data into place (i.e. `data_path` and `index_path`)"] # [doc = " and is expected to be removed by the caller when ready."] pub keep_path : Option < PathBuf > , }
};
}
