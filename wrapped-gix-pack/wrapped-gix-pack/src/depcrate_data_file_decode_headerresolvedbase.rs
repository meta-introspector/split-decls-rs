// Generated macro for ResolvedBase (enum)
macro_rules! Depcrate_data_file_decode_headerResolvedBase {
() => {
// Module: crate::data::file::decode::header
// Provides: {"ResolvedBase"}
// Dependencies: {}
# [doc = " A return value of a resolve function, which given an [`ObjectId`][gix_hash::ObjectId] determines where an object can be found."] # [derive (Debug , PartialEq , Eq , Hash , Ord , PartialOrd , Clone)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub enum ResolvedBase { # [doc = " Indicate an object is within this pack, at the given entry, and thus can be looked up locally."] InPack (data :: Entry) , # [doc = " Indicates the object of `kind` was found outside of the pack."] OutOfPack { # [doc = " The kind of object we found when reading the header of the out-of-pack base."] kind : gix_object :: Kind , # [doc = " The amount of deltas encountered if the object was packed as well."] num_deltas : Option < u32 > , } , }
};
}
